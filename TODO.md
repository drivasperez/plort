- Unicode renderer using braille characters instead of ascii. (default terminal renderer)
- Put renderers into a common Renderer trait.
- Unify colour-themes between ascii and svg
- Different default dimensions for ascii (chars) and svg (pixels)
- Generally fix up flags/options to make sense
- SVG: Header with key
- SVG: Labelled axes
- Read theme / defaults out of a config file.
- General refactor
- Better error for log on negative values
- Make column usize instead of u8
