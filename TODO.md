- Put renderers into a common Renderer trait.
- Unify colour-themes between ascii and svg
- Different default dimensions for ascii (chars) and svg (pixels)
- Generally fix up flags/options to make sense
- SVG: Header with key
- SVG: Labelled axes
- Unicode renderer using braille characters instead of ascii. (default terminal renderer)
- Snapshot tests of CLI output
- Read theme / defaults out of a config file.
- General refactor