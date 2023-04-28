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

    #[clap(long)]
    pub width: usize,
    #[clap(long)]
    pub height: usize,

    #[clap(short, long)]
    pub mode: PlotType,
    #[clap(short, long = "output")]
    pub output_type: OutputType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlotType {
    Dot,
    Line,
    Count,
}

impl Default for PlotType {
    fn default() -> Self {
        PlotType::Dot
    }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutputType {
    Ascii,
    Svg,
}

impl Default for OutputType {
    fn default() -> Self {
        OutputType::Ascii
    }
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
