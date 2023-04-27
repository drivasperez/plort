use crate::config::Config;
use crate::types::DataSet;

pub struct PlotInfo {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,

    pub x_range: f64,
    pub y_range: f64,

    pub log_x: bool,
    pub log_y: bool,

    pub width: u32,
    pub height: u32,

    pub draw_x_axis: bool,
    pub draw_y_axis: bool,
    pub y_axis: u32,
    pub x_axis: u32,
}

pub fn draw(_config: &Config, _dataset: &DataSet) -> anyhow::Result<()> {
    todo!()
}
