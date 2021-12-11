use std::{env::args, io};

fn main() -> io::Result<()> {
    let file_name = args()
        .nth(1)
        .expect("Expected file path to the scenario to play.");
    let scenario = content::parser::read_yaml(&file_name)?;

    tui::run(app::app(scenario));

    Ok(())
}

mod app {
    use crate::content::Modifier;
    use crate::{
        content::{self, Id, Scenario},
        tui::*,
    };

    pub struct Model {
        scenario: Scenario,
        current: Id,
    }

    pub fn app(scenario: Scenario) -> App<Model> {
        let current = scenario.first.clone();

        App {
            model: Model { scenario, current },
            update,
            view,
        }
    }

    pub fn update(event: Event, model: Model) -> Option<Model> {
        match event {
            Event::Key(Key::Esc) => None,
            Event::Key(Key::Char(c)) => advance(model, content::Key::from(c)),
            Event::Key(Key::Alt(c)) => {
                advance(model, content::Key::with_modifier(c, Modifier::Alt))
            }
            Event::Key(Key::Ctrl(c)) => {
                advance(model, content::Key::with_modifier(c, Modifier::Ctrl))
            }
            _ => Some(model),
        }
    }

    fn advance(mut model: Model, key: content::Key) -> Option<Model> {
        model.current = model
            .scenario
            .advance(&model.current, &key)
            .unwrap_or(model.current);
        Some(model)
    }

    pub fn view(model: &Model) -> UI {
        UI::Content(model.scenario.get(&model.current).unwrap_or(""))
    }
}

pub mod content {
    use serde::Deserialize;
    use std::collections::{BTreeMap, BTreeSet};

    pub struct Scenario {
        pub first: Id,
        screens: BTreeMap<Id, Screen>,
        transitions: Vec<Transition>,
    }

    pub type Id = String;
    pub type Screen = String;

    pub struct Transition {
        from: Id,
        to: Id,
        key: Key,
    }

    #[derive(Clone, PartialEq, Eq)]
    pub struct Key {
        key: char,
        modifiers: BTreeSet<Modifier>,
    }

    #[derive(Deserialize, Clone, PartialOrd, Ord, PartialEq, Eq)]
    pub enum Modifier {
        Alt,
        Ctrl,
    }

    impl Scenario {
        pub fn get(&self, pos: &Id) -> Option<&str> {
            self.screens.get(pos).map(String::as_ref)
        }

        pub fn advance(&self, pos: &Id, input: &Key) -> Option<Id> {
            self.transitions
                .iter()
                .filter(|Transition { from, key, .. }| from == pos && key == input)
                .map(|Transition { to, .. }| to.clone())
                .nth(0)
        }
    }

    impl Key {
        pub fn from(key: char) -> Self {
            Self {
                key,
                modifiers: BTreeSet::new(),
            }
        }

        pub fn with_modifier(key: char, modifier: Modifier) -> Self {
            Self {
                key,
                modifiers: BTreeSet::from([modifier]),
            }
        }
    }

    pub mod parser {
        use super::*;
        use combine::parser::char::{char, string};
        use combine::{attempt, choice, satisfy, Parser};
        use serde::de::{self, Visitor};
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use std::collections::BTreeSet;
        use std::{fmt, fs, io, path::Path};

        #[derive(Deserialize)]
        pub struct FileScenario {
            first: Id,
            screens: BTreeMap<Id, Screen>,
            transitions: Vec<FileTransition>,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        pub enum FileTransition {
            Key { from: Id, to: Id, key: Key },
            Keys { from: Id, to: Id, keys: Vec<Key> },
            // Word { from: Id, to: Id, word: Word },
        }

        // pub type Word = String;

        pub fn read_yaml(path: impl AsRef<Path>) -> io::Result<Scenario> {
            use FileTransition as FT;

            let content: String = fs::read_to_string(path)?;
            let FileScenario {
                first,
                screens,
                transitions,
            } = serde_yaml::from_str(&content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            // add_intermediate_word_screens(&mut screens, &transitions);

            Ok(Scenario {
                first,
                screens,
                transitions: transitions
                    .iter()
                    .flat_map(|t| match t {
                        FT::Key { from, to, key } => file_key_to_transition(from, to, key),
                        FT::Keys { from, to, keys } => file_keys_to_transition(from, to, keys),
                        // FT::Word { from, to, word } => file_word_to_transition(from, to, word),
                    })
                    .collect::<Vec<_>>(),
            })
        }

        // fn add_intermediate_word_screens(
        //     screens: &mut BTreeMap<Id, Screen>,
        //     transitions: Vec<Transition>,
        // ) {
        // }

        fn file_key_to_transition(from: &Id, to: &Id, key: &Key) -> Vec<Transition> {
            vec![Transition {
                from: from.into(),
                to: to.into(),
                key: key.clone(),
            }]
        }

        fn file_keys_to_transition(from: &Id, to: &Id, keys: &Vec<Key>) -> Vec<Transition> {
            keys.iter()
                .map(|key| Transition {
                    from: from.into(),
                    to: to.into(),
                    key: key.clone(),
                })
                .collect::<Vec<_>>()
        }

        // fn file_word_to_transition(from: &Id, to: &Id, word: &Word) -> Vec<Transition> {
        //     let first = 0;
        //     let last = word.len() - 1;
        //
        //     word.chars()
        //         .enumerate()
        //         .map(|(index, key)| Transition {
        //             from: if index == first {
        //                 format!("{}", from)
        //             } else {
        //                 format!("{}-{}", from, index - 1)
        //             },
        //             to: if index == last {
        //                 format!("{}", to)
        //             } else {
        //                 format!("{}-{}", from, index)
        //             },
        //             key: Key {
        //                 key,
        //                 modifiers: BTreeSet::new(),
        //             },
        //         })
        //         .collect::<Vec<_>>()
        // }

        impl Key {
            fn to_string(&self) -> String {
                self.key.into()
            }

            fn from_string(str: &str) -> Result<Self, ()> {
                choice((
                    attempt(
                        (
                            char('<'),
                            choice((
                                attempt(char('A')).map(|_| Modifier::Alt),
                                attempt(char('C')).map(|_| Modifier::Ctrl),
                            )),
                            char('-'),
                            satisfy(|_| true),
                            char('>'),
                        )
                            .map(|(_, modifier, _, key, _)| Key {
                                key,
                                modifiers: BTreeSet::from([modifier]),
                            }),
                    ),
                    attempt(
                        choice((
                            attempt(string("<Enter>").map(|_| '\n')),
                            attempt(satisfy(|_| true)),
                        ))
                        .map(|key| Key {
                            key,
                            modifiers: BTreeSet::new(),
                        }),
                    ),
                ))
                .parse(str)
                .map(|(key, _)| key)
                .map_err(|_| ())
            }
        }

        impl Serialize for Key {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for Key {
            fn deserialize<D>(deserializer: D) -> Result<Key, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(KeyStringVisitor)
            }
        }

        struct KeyStringVisitor;
        impl<'de> Visitor<'de> for KeyStringVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Key")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Key::from_string(value).map_err(|_| E::custom(format!("Invalid key: {}", value)))
            }
        }
    }
}

pub mod tui {
    use std::{
        io::{stdin, stdout, Write},
        sync::mpsc::{channel, Receiver},
        thread,
    };

    pub use termion::event::{Event, Key, MouseButton, MouseEvent};
    use termion::{
        cursor::HideCursor,
        input::{MouseTerminal, TermRead},
        raw::IntoRawMode,
        screen::AlternateScreen,
    };

    pub struct App<Model: Sized> {
        pub model: Model,
        pub update: fn(Event, Model) -> Option<Model>,
        pub view: fn(&Model) -> UI,
    }

    pub enum UI<'a> {
        Content(&'a str),
    }

    pub fn run<Model>(app: App<Model>) {
        let App {
            mut model,
            update,
            view,
        } = app;

        let mut out = init_tui();
        let receiver = listen_events();

        render(view(&model), &mut out);

        loop {
            match wait_for_event(&receiver) {
                Some(event) => {
                    model = match update(event, model) {
                        Some(new_model) => new_model,
                        None => break,
                    };
                    render(view(&model), &mut out);
                }
                _ => break,
            }
        }
    }

    fn init_tui() -> impl Write {
        MouseTerminal::from(HideCursor::from(AlternateScreen::from(
            stdout().into_raw_mode().unwrap(),
        )))
    }

    fn listen_events() -> Receiver<Event> {
        let (sender, receiver) = channel();

        thread::spawn(move || {
            stdin()
                .events()
                .filter_map(Result::ok)
                .try_for_each(|e| sender.send(e))
        });

        receiver
    }

    fn wait_for_event(receiver: &Receiver<Event>) -> Option<Event> {
        receiver.recv().ok()
    }

    fn render(ui: UI, out: &mut impl Write) {
        write!(out, "{}{}", termion::clear::All, termion::style::Bold);

        match ui {
            UI::Content(content) => content.lines().enumerate().for_each(|(pos, content)| {
                write!(
                    out,
                    "{}{}",
                    termion::cursor::Goto(1, (pos + 1) as u16),
                    &content
                )
                .unwrap()
            }),
        }

        out.flush().unwrap();
    }
}
