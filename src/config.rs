use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug, Default)]
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

    #[clap(long, short)]
    pub dimensions: Dimensions,

    #[clap(short, long, default_value = "dot")]
    pub mode: PlotType,
    #[clap(short, long = "output", default_value = "ascii")]
    pub output_type: OutputType,

    #[clap(short = 'A', long, default_value = "true")]
    pub axis: bool,
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

        let width = w.parse::<usize>().map_err(|e| {
            format!(
                "Invalid width: {}. Expected an integer value.",
                e
            )
        })?;

        let height = h.parse::<usize>().map_err(|e| {
            format!(
                "Invalid height: {}. Expected an integer value.",
                e
            )
        })?;

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
