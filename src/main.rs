use std::{env::args, io};

fn main() -> io::Result<()> {
    let file_name = args()
        .nth(1)
        .expect("Expected file path to the scenario to play.");
    let content = content::parser::read_markdown(&file_name)?;

    tui::run(app::app(content));

    Ok(())
}

mod app {
    use std::cmp::min;
    use crate::{content::Content, tui::*};

    pub struct Model {
        content: Vec<Content>,
        pos:     usize,
    }

    pub fn app(content: Vec<Content>) -> App<Model> {
        App {
            model: Model { content, pos: 0 },
            update,
            view,
        }
    }

    pub fn update(event: Event, mut model: Model) -> Option<Model> {
        match event {
            Event::Key(Key::Esc) => None,
            Event::Key(Key::Char(c)) => {
                let index = min(model.pos, model.content.len() - 1);

                if model.content[index].key.contains(c) {
                    model.pos += 1;
                }

                Some(model)
            },
            _ => Some(model),
        }
    }

    pub fn view(model: &Model) -> UI {
        let index = min(model.pos, model.content.len() - 1);
        let screen = &model.content[index].screen;

        UI::Content(screen.into())
    }
}

pub mod content {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Content {
        pub screen: String,
        pub key:    String,
    }

    impl Content {
        pub fn new(screen: String, key: String) -> Self {
            Self { screen, key }
        }
    }

    pub mod parser {
        use std::{fs::read_to_string, io, path::Path};

        use markdown::{
            self,
            Block::{self, *},
            Span::{self, *},
        };

        use super::Content;

        pub fn read_markdown(path: impl AsRef<Path>) -> io::Result<Vec<Content>> {
            let markdown = &read_to_string(path)?;
            let tokens = markdown::tokenize(&markdown);

            let screens = tokens
                .iter()
                .flat_map(markdown_block_to_screen)
                .collect::<Vec<_>>();

            let keys = tokens
                .iter()
                .flat_map(markdown_block_to_key)
                .collect::<Vec<_>>();

            let content = screens
                .iter()
                .zip(keys.iter())
                .map(|(screen, key)| Content::new(screen.into(), key.into()))
                .collect::<Vec<_>>();

            Ok(content)
        }

        fn markdown_block_to_screen(block: &Block) -> Vec<String> {
            match block {
                CodeBlock(Some(lang), content) if lang == "screen" => vec![content.into()],
                _ => vec![],
            }
        }

        fn markdown_block_to_key(block: &Block) -> Vec<String> {
            match block {
                Paragraph(content) => content
                    .iter()
                    .filter_map(markdown_span_to_key)
                    .collect::<Vec<_>>(),
                _ => vec![],
            }
        }

        fn markdown_span_to_key(span: &Span) -> Option<String> {
            match span {
                Code(content) => Some(content.into()),
                _ => None,
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
        pub model:  Model,
        pub update: fn(Event, Model) -> Option<Model>,
        pub view:   fn(&Model) -> UI,
    }

    pub enum UI {
        Content(String),
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
                },
                _ => break,
            }
        }
    }

    fn init_tui() -> impl Write {
        MouseTerminal::from(HideCursor::from(AlternateScreen::from(
            stdout().into_raw_mode().unwrap()
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
