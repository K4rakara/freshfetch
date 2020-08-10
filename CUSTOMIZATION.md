# Customization

Customizing Freshfetch is done through a special markup format called CLML. To
get started with customizing Freshfetch, copy the example config files (
`info.clml`, `layout.clml`, `art.clml`) from `/usr/share/freshfetch/` to
`~/.local/share/freshfetch`.

## Intro to CLML

CLML is meant to save time when writing CLI utilities without an impact on
performance. As such, its tags are (for the most part) directly converted to
ANSI escape codes. For example, `<forward 25>` becomes `\e[25C`.

// TODO: CLML manual