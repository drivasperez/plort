use clap::Parser;
use std::{path::PathBuf, str::FromStr};

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(long = "flip-xy")]
    pub flip_xy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlotType {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutputType {
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
