```screen
[_]
```
`c`
```screen
c : _
[c_] = _
```
`l`
```screen
cl : _
[cl_] = _
```
`a`
```screen
cla : _
[cla_] = _
```
`m`
```screen
clam : _
[clam_] = _
```
`p`
```screen
clamp : _
[clamp_] = _
```
` =`
```screen
clamp : _
clamp = [_]
```
`\`
```screen
clamp : _ -> _
clamp = \[_] ->
    _
```
`v`
```screen
clamp : _ -> _
clamp = \[v_] ->
    _
```
` ,`
```screen
clamp : _ -> _
clamp = \v, [_] ->
    _
```
`l`
```screen
clamp : _, _ -> _
clamp = \v, [l_] ->
    _
```
`o`
```screen
clamp : _, _ -> _
clamp = \v, [lo_] ->
    _
```
`w`
```screen
clamp : _, _ -> _
clamp = \v, [low_] ->
    _
```
` ,`
```screen
clamp : _, _ -> _
clamp = \v, low, [_] ->
    _
```
`h`
```screen
clamp : _, _, _ -> _
clamp = \v, low, [h_] ->
    _
```
`i`
```screen
clamp : _, _, _ -> _
clamp = \v, low, [hi_] ->
    _
```
`g`
```screen
clamp : _, _, _ -> _
clamp = \v, low, [hig_] ->
    _
```
`h`
```screen
clamp : _, _, _ -> _
clamp = \v, low, [high_] ->
    _
```
` ,`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high, [_] ->
    _
```
` ->`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    [_]
```
`v`
```screen
clamp : a, *, * -> a
clamp = \v, low, high ->
    [v_]
```
` `
```screen
clamp : a, *, * -> a
clamp = \v, low, high ->
    v [_]
```
`|`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> [_]
```
`m`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> [m_]
```
`a`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> [ma_]
```
`x`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> [max_]
```
` `
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max [_]
```
`l`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max [l_]
```
`o`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max [lo_]
```
`w`
```screen
clamp : Num *, Num *, * -> Num *
clamp = \v, low, high ->
    v
        |> max [low_]
```
` `
```screen
clamp : Num *, Num *, * -> Num *
clamp = \v, low, high ->
    v
        |> max low [_]
```
`|`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max low
        |> [_]
```
`m`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max low
        |> [m_]
```
`i`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max low
        |> [mi_]
```
`n`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max low
        |> [min_]
```
` `
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max low
        |> min [_]
```
`h`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max low
        |> min [h_]
```
`i`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max low
        |> min [hi_]
```
`g`
```screen
clamp : _, _, _ -> _
clamp = \v, low, high ->
    v
        |> max low
        |> min [hig_]
```
`h`
```screen
clamp : Num *, Num *, Num * -> Num *
clamp = \v, low, high ->
    v
        |> max low
        |> min [high_]
```
` `
