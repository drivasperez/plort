pub mod ascii;
pub mod braille;
mod braille_point;

// When drawing a text chart, we need to:
// - Calculate the dimensions of the chart based on the width of the dataset and the
//   character set used to draw the chart. Braille characters can be two points
//   wide and four points tall. This means that a braille chart can be half the
//   physical width and a quarter the physical height of an ASCII chart.
//
// - Print a header for the chart showing the bounds of the dataset and a legend
//   for the chart showing the characters/colors used to draw the chart series.
//
// - Set up a string buffer to hold the chart. The string buffer will be a string
//   of length `width * height` where `width` and `height` are the dimensions of
//   the chart. The string buffer will be initialized with the background character
//   for the chart.
//
// - Draw the chart axes. The chart axes will be drawn using the character set
//   specified by the user. The chart axes will be drawn using the foreground
//   color specified by the theme.
//
// - Draw the chart series. The chart series will be drawn using the character
//   set specified by the user. The chart series will be drawn using the foreground
//   colors specified by the theme.
//
// - Print the chart to the terminal. The chart will be printed to the terminal
//   using the character set specified by the user.
//
// When drawing a braille chart specifically, we need to:
//
// - Scale axes appropriately. Braille characters are two points wide and four points
//   tall.
//
// - Read in the data set in chunks of 2x4 points. Each chunk of 2x4 points will
//   be mapped to a single braille character.
//
//
