use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug, Default)]
#[clap(version, author, about)]
pub struct Config {
    #[clap(long = "flip-xy")]
    pub flip_xy: bool,
    #[clap(short, long)]
    pub stream_mode: bool,
    #[clap(long)]
    pub x_column: bool,

    #[clap(long)]
    pub log_x: bool,
    #[clap(long)]
    pub log_y: bool,

    #[clap(long, short, default_value = "80x40")]
    pub dimensions: Dimensions,

    #[clap(short, long, default_value = "dot")]
    pub mode: PlotType,
    #[clap(short, long = "output", default_value = "ascii")]
    pub output_type: OutputType,

    #[clap(short = 'A', long, default_value = "true")]
    pub axis: bool,

    #[clap(long = "colors", default_value = "bank-wong")]
    pub color_scheme: ColorScheme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Default for Dimensions {
    fn default() -> Self {
        Dimensions {
            width: 72,
            height: 40,
        }
    }
}

impl FromStr for Dimensions {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (w, h) = s.split_once('x').ok_or_else(|| {
            format!(
                "Invalid dimensions: {}. Expected format: <width>x<height>",
                s
            )
        })?;

        let width = w
            .parse::<usize>()
            .map_err(|e| format!("Invalid width: {}. Expected an integer value.", e))?;

        let height = h
            .parse::<usize>()
            .map_err(|e| format!("Invalid height: {}. Expected an integer value.", e))?;

        Ok(Dimensions { width, height })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlotType {
    #[default]
    Dot,
    Line,
    Count,
}

impl FromStr for PlotType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dot" => Ok(PlotType::Dot),
            "line" => Ok(PlotType::Line),
            "count" => Ok(PlotType::Count),
            _ => Err(format!("Unknown plot type: {}", s)),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum OutputType {
    #[default]
    Ascii,
    Svg,
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascii" => Ok(OutputType::Ascii),
            "svg" => Ok(OutputType::Svg),
            _ => Err(format!("Unknown output type: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorScheme {
    axis: (u8, u8, u8),
    series: &'static [(u8, u8, u8)],
}

impl ColorScheme {
    pub fn series_color(&self, col: u8) -> (u8, u8, u8) {
        self.series[col as usize % self.series.len()]
    }

    pub fn axis_color(&self) -> (u8, u8, u8) {
        self.axis
    }
}

impl FromStr for ColorScheme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bank-wong" => Ok(BANK_WONG_COLOR_SCHEME),
            "mono-light" => Ok(MONO_LIGHT_SCHEME),
            "mono-dark" => Ok(MONO_DARK_SCHEME),

            _ => Err(format!("Unknown color scheme: {}", s)),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        BANK_WONG_COLOR_SCHEME
    }
}

pub const BANK_WONG_COLOR_SCHEME: ColorScheme = {
    ColorScheme {
        axis: (123, 123, 125),
        series: &[
            (0, 114, 178),
            (230, 159, 0),
            (86, 180, 233),
            (0, 158, 115),
            (240, 228, 66),
            (0, 0, 0),
            (213, 94, 0),
            (204, 121, 167),
        ],
    }
};

pub const MONO_LIGHT_SCHEME: ColorScheme = {
    ColorScheme {
        axis: (255, 255, 255),
        series: &[(255, 255, 255)],
    }
};

pub const MONO_DARK_SCHEME: ColorScheme = {
    ColorScheme {
        axis: (0, 0, 0),
        series: &[(0, 0, 0)],
    }
};
