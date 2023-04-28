use crate::config::{Config, OutputType, PlotType};
use crate::scale::ScaledPoint;
use crate::scale::TransformType;
use crate::types::DataSet;
use crate::types::{Point, EMPTY_VALUE};

const CROSS_PAD: f64 = 2.0;

pub struct PlotInfo {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,

    pub x_range: f64,
    pub y_range: f64,

    pub log_x: bool,
    pub log_y: bool,

    pub width: usize,
    pub height: usize,

    pub draw_x_axis: bool,
    pub draw_y_axis: bool,
    pub y_axis: usize,
    pub x_axis: usize,
}

impl PlotInfo {
    pub fn draw_calc_bounds(&mut self, dataset: &DataSet) {
        let mut min_point = Point(f64::MAX, f64::MAX);
        let mut max_point = Point(f64::MIN, f64::MIN);

        for col in 0..dataset.columns {
            for row in 0..dataset.rows {
                let point = &dataset.points[col as usize][row];
                if point.0 < min_point.0 {
                    min_point.0 = point.0;
                }
                if point.1 < min_point.1 {
                    min_point.1 = point.1;
                }
                if point.0 > max_point.0 {
                    max_point.0 = point.0;
                }
                if point.1 > max_point.1 {
                    max_point.1 = point.1;
                }
            }
        }

        let transform = TransformType::new(self.log_x, self.log_y);
        let mut min_point = ScaledPoint::new(min_point, self, transform);
        let mut max_point = ScaledPoint::new(max_point, self, transform);

        if (min_point.0 - max_point.0).abs() < 1 {
            min_point.0 -= 1;
            max_point.0 += 1;
        }

        if (min_point.1 - max_point.1).abs() < 1 {
            min_point.1 -= 1;
            max_point.1 += 1;
        }

        self.x_min = min_point.0 as f64;
        self.x_max = max_point.0 as f64;
        self.y_min = min_point.1 as f64;
        self.y_max = max_point.1 as f64;

        self.x_range = self.x_max - self.x_min;
        self.y_range = self.y_max - self.y_min;

        let crosses_x_axis = self.y_min < 0.0 && self.y_max > 0.0;
        let crosses_y_axis = self.x_min < 0.0 && self.x_max > 0.0;

        // If the data does not cross the x or y axis, we can
        // clamp the plot's axis to zero.
        if !crosses_x_axis {
            if 0.0 < self.x_min && 0.0 > self.x_min - self.x_range * CROSS_PAD {
                self.x_min = 0.0;
                self.x_range = self.x_max;
            } else if 0.0 > self.x_max && 0.0 < self.x_max + self.x_range * CROSS_PAD {
                self.x_max = 0.0;
                self.x_range = -self.x_min;
            }
        }

        if !crosses_y_axis {
            if 0.0 < self.y_min && 0.0 > self.y_min - self.y_range * CROSS_PAD {
                self.y_min = 0.0;
                self.y_range = self.y_max;
            } else if 0.0 > self.y_max && 0.0 < self.y_max + self.y_range * CROSS_PAD {
                self.y_max = 0.0;
                self.y_range = -self.y_min;
            }
        }
    }

    fn all_empty_points(&self) -> bool {
        // ??? Why does this mean all empty points?
        // A: Because the default values are f64::MAX
        self.x_min == f64::MAX || self.y_min == f64::MAX
    }

    fn insufficient_range(&self) -> bool {
        self.x_range == 0.0 || self.y_range == 0.0
    }
}

impl Default for PlotInfo {
    fn default() -> Self {
        PlotInfo {
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            x_range: 0.0,
            y_range: 0.0,
            log_x: false,
            log_y: false,
            width: 0,
            height: 0,
            draw_x_axis: false,
            draw_y_axis: false,
            y_axis: 0,
            x_axis: 0,
        }
    }
}

pub fn draw(config: &Config, dataset: &DataSet) -> anyhow::Result<()> {
    let mut plot_info = PlotInfo::default();
    plot_info.log_x = config.log_x;
    plot_info.log_y = config.log_y;

    plot_info.draw_calc_bounds(dataset);

    if plot_info.all_empty_points() {
        return Ok(());
    }

    if plot_info.insufficient_range() {
        return Ok(());
    }

    plot_info.width = config.width;
    plot_info.height = config.height;

    if config.mode == PlotType::Count {
        // Count mode
    }

    match config.output_type {
        OutputType::Ascii => draw_ascii(config, dataset, &plot_info),
        OutputType::Svg => draw_svg(config, dataset, &plot_info),
    }

    Ok(())
}

fn draw_ascii(config: &Config, dataset: &DataSet, plot_info: &PlotInfo) {}

fn draw_svg(config: &Config, dataset: &DataSet, plot_info: &PlotInfo) {}
