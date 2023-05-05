use std::collections::HashMap;

use crate::config::{Config, OutputType, PlotType};
use crate::scale::ScaledPoint;
use crate::scale::TransformType;
use crate::types::DataSet;
use crate::types::Point;
use svg::{svg_plot, SvgTheme};
use text::ascii::ascii_plot;
use text::braille::braille_plot;

mod svg;
mod text;

const CROSS_PAD: f64 = 2.0;

#[derive(Debug, Clone)]
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

    pub counters: Option<Vec<HashMap<(i32, i32), u32>>>,
}

impl PlotInfo {
    pub fn draw_calc_axis_pos(&mut self) {
        self.draw_x_axis = 0.0 >= self.y_min && 0.0 <= self.y_max;
        self.draw_y_axis = 0.0 >= self.x_min && 0.0 <= self.x_max;

        let mut origin = Point(0.0, 0.0);
        if !self.draw_y_axis {
            if 0.0 < self.x_min {
                origin.0 = self.x_min;
            } else {
                origin.0 = self.x_max;
            }
        }

        if !self.draw_x_axis {
            if 0.0 < self.y_min {
                origin.1 = self.y_min;
            } else {
                origin.1 = self.y_max;
            }
        }

        let sp = ScaledPoint::new(origin, self, TransformType::new(self.log_x, self.log_y));
        // XXX Is this safe? SP are signed, but x_axis and y_axis are unsigned.
        self.x_axis = sp.0 as usize;
        self.y_axis = sp.1 as usize;
    }

    pub fn draw_calc_bounds(&mut self, dataset: &DataSet) {
        let mut min_point = Point(f64::MAX, f64::MAX);
        let mut max_point = Point(f64::MIN, f64::MIN);

        for col in 0..dataset.columns {
            for row in 0..dataset.rows {
                let point = dataset.points[col as usize][row];
                if point.is_empty() {
                    continue;
                }

                if self.log_x && point.0 <= 0.0 {
                    panic!("Log scale requires positive values");
                }

                if self.log_y && point.1 <= 0.0 {
                    panic!("Log scale requires positive values");
                }

                let x = point.0;
                let y = point.1;

                if x < min_point.0 {
                    min_point.0 = x;
                }
                if x > max_point.0 {
                    max_point.0 = x;
                }

                if y < min_point.1 {
                    min_point.1 = y;
                }
                if y > max_point.1 {
                    max_point.1 = y;
                }
            }
        }

        let transform = TransformType::new(self.log_x, self.log_y);
        let min_point = min_point.scale_transform(transform);
        let mut max_point = max_point.scale_transform(transform);

        if min_point.0 == max_point.0 {
            max_point.0 += 1.0;
        }

        if min_point.1 == max_point.1 {
            max_point.1 += 1.0;
        }

        self.x_min = min_point.0;
        self.x_max = max_point.0;
        self.y_min = min_point.1;
        self.y_max = max_point.1;

        self.x_range = self.x_max - self.x_min;
        self.y_range = self.y_max - self.y_min;

        let crosses_x_axis = self.x_min <= 0.0 && self.x_max >= 0.0;
        let crosses_y_axis = self.y_min <= 0.0 && self.y_max >= 0.0;

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
            counters: None,
        }
    }
}

pub fn draw(config: &Config, dataset: &DataSet) -> anyhow::Result<()> {
    let mut plot_info = PlotInfo {
        log_x: config.log_x,
        log_y: config.log_y,
        ..Default::default()
    };

    plot_info.draw_calc_bounds(dataset);

    if plot_info.all_empty_points() {
        return Ok(());
    }

    if plot_info.insufficient_range() {
        return Ok(());
    }

    plot_info.width = config.dimensions.width;
    plot_info.height = config.dimensions.height;

    if config.mode == PlotType::Count {
        let mut counters = Vec::new();
        for col in 0..dataset.columns {
            counters.push(count_points(dataset, &plot_info, col as usize));
        }

        plot_info.counters = Some(counters);
    }

    match config.output_type {
        OutputType::Ascii => ascii_plot(config, dataset, &mut plot_info),
        OutputType::Braille => braille_plot(config, dataset, &mut plot_info),
        OutputType::Svg => {
            let theme: SvgTheme = SvgTheme {
                line_width: 2.0,
                border_width: 2.0,
                axis_width: 2.0,
                bg_color: "black".into(),
                border_color: "white".into(),
                axis_color: "lightgray".into(),
                colors: vec![
                    "#377eb8".into(),
                    "#e41a1c".into(),
                    "#4daf4a".into(),
                    "#984ea3".into(),
                    "#ff7f00".into(),
                    "#ffff33".into(),
                    "#a65628".into(),
                    "#f781bf".into(),
                    "#999999".into(),
                ],
            };

            svg_plot(config, &mut plot_info, dataset, &theme);
        }
    }

    Ok(())
}

fn count_points(dataset: &DataSet, plot_info: &PlotInfo, col: usize) -> HashMap<(i32, i32), u32> {
    let points = &dataset.points[col];
    let transform = TransformType::new(plot_info.log_x, plot_info.log_y);
    let mut counts = HashMap::new();

    for point in points.iter().take(dataset.rows) {
        if point.0.is_nan() || point.1.is_nan() {
            continue;
        }

        let scaled_point = ScaledPoint::new(*point, plot_info, transform);
        let x = scaled_point.0;
        let y = scaled_point.1;

        let count = counts.entry((x, y)).or_insert(0);
        *count += 1;
    }

    counts
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::{process_line, ProcessLineResult};

    fn read_lines(cfg: &Config, dataset: &mut DataSet, lines: &[&str]) -> PlotInfo {
        let mut plot_info = PlotInfo::default();

        plot_info.log_x = cfg.log_x;
        plot_info.log_y = cfg.log_y;

        for (i, line) in lines.iter().enumerate() {
            assert_eq!(ProcessLineResult::Ok, process_line(cfg, dataset, line, i));
        }

        plot_info
    }

    #[test]
    fn bounds_basic() {
        let cfg = Config::default();
        let mut dataset = DataSet::default();
        let lines = ["1 2 3", "4 5 6", "7 8 9"];

        let mut plot_info = PlotInfo::default();

        read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        dbg!(&plot_info.x_min);
        dbg!(&plot_info.x_max);
        dbg!(&plot_info.y_min);
        dbg!(&plot_info.y_max);

        assert_eq!(plot_info.x_min, 0.0);
        assert_eq!(plot_info.x_max, 2.0);
        assert_eq!(plot_info.y_min, 0.0);
        assert_eq!(plot_info.y_max, 9.0);
    }

    #[test]
    fn bounds_empty_cells() {
        let cfg = Config::default();
        let mut dataset = DataSet::default();
        let lines = ["  2 3", "4   6", "7 8  "];

        let mut plot_info = PlotInfo::default();

        read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        dbg!(&plot_info.x_min);
        dbg!(&plot_info.x_max);
        dbg!(&plot_info.y_min);
        dbg!(&plot_info.y_max);

        assert_eq!(plot_info.x_min, 0.0);
        assert_eq!(plot_info.x_max, 2.0);
        assert_eq!(plot_info.y_min, 0.0);
        assert_eq!(plot_info.y_max, 8.0);
    }

    #[test]
    fn bounds_negative_quadrant() {
        let mut cfg = Config::default();
        cfg.x_column = true;
        let mut dataset = DataSet::default();
        let lines = ["-5 -50", "-3 -30", "-1 -10"];

        let mut plot_info = PlotInfo::default();

        read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        dbg!(&plot_info.x_min);
        dbg!(&plot_info.x_max);
        dbg!(&plot_info.y_min);
        dbg!(&plot_info.y_max);

        assert_eq!(plot_info.x_min, -5.0);
        assert_eq!(plot_info.x_max, 0.0);
        assert_eq!(plot_info.y_min, -50.0);
        assert_eq!(plot_info.y_max, 0.0);
    }

    #[test]
    fn bounds_4q() {
        let mut cfg = Config::default();
        cfg.x_column = true;
        let mut dataset = DataSet::default();
        let lines = ["-5 -50", "-49 49", "0 0", "10 10"];

        let mut plot_info = PlotInfo::default();

        read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        dbg!(&plot_info.x_min);
        dbg!(&plot_info.x_max);
        dbg!(&plot_info.y_min);
        dbg!(&plot_info.y_max);

        // In the original test it's -50 but it seems like it should be -49?
        assert_eq!(plot_info.x_min, -49.0);
        assert_eq!(plot_info.x_max, 10.0);
        assert_eq!(plot_info.y_min, -50.0);
        assert_eq!(plot_info.y_max, 49.0);
    }

    #[test]
    fn bounds_diff_quadrants_for_columns() {
        let mut cfg = Config::default();
        cfg.x_column = true;
        let mut dataset = DataSet::default();
        let lines = ["-50 -50 50", "-49 -49 49", "-48 -48 48", "-47 -47 47"];

        let mut plot_info = PlotInfo::default();

        read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        dbg!(&plot_info.x_min);
        dbg!(&plot_info.x_max);
        dbg!(&plot_info.y_min);
        dbg!(&plot_info.y_max);

        assert_eq!(plot_info.x_min, -50.0);
        assert_eq!(plot_info.x_max, -47.0);
        assert_eq!(plot_info.y_min, -50.0);
        assert_eq!(plot_info.y_max, 50.0);
    }

    #[test]
    fn bounds_too_distant_to_touch_axis() {
        let mut cfg = Config::default();
        cfg.x_column = true;
        let mut dataset = DataSet::default();
        let lines = ["-5000 -5000", "-4900 -4900", "-4800 -4800"];

        let mut plot_info = PlotInfo::default();

        read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        dbg!(&plot_info.x_min);
        dbg!(&plot_info.x_max);
        dbg!(&plot_info.y_min);
        dbg!(&plot_info.y_max);

        assert_eq!(plot_info.x_min, -5000.0);
        assert_eq!(plot_info.x_max, -4800.0);
        assert_eq!(plot_info.y_min, -5000.0);
        assert_eq!(plot_info.y_max, -4800.0);
    }

    #[test]
    fn bounds_log_basic() {
        let mut cfg = Config::default();
        cfg.log_y = true;

        let lines = ["1214", "358", "316", "187", "186", "93", "63", "11"];

        let mut dataset = DataSet::default();

        let mut plot_info = read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        assert_eq!(plot_info.x_min, 0.0);
        assert_eq!(plot_info.x_max, 7.0);

        assert_eq!(plot_info.y_min, 0.0);
        // Assert that it's close enough to 7.101
        assert!((plot_info.y_max - 7.101).abs() < 0.001);
    }

    #[test]
    fn bounds_log_distant() {
        let mut cfg = Config::default();
        cfg.log_y = true;

        let lines = ["1214", "358", "316", "187", "186"];

        let mut dataset = DataSet::default();

        let mut plot_info = read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        assert_eq!(plot_info.x_min, 0.0);
        assert_eq!(plot_info.x_max, 4.0);

        assert!((plot_info.y_min - 5.2257).abs() < 0.001);
        assert!((plot_info.y_max - 7.101).abs() < 0.001);
    }

    #[test]
    fn reject_x_range_of_zero() {
        let cfg = Config::default();

        let lines = ["-3 -2"];

        let mut dataset = DataSet::default();

        let mut plot_info = read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        assert_ne!(plot_info.y_range, 0.0);
    }

    #[test]
    fn reject_y_range_of_zero() {
        let cfg = Config::default();

        let lines = ["0 0", "0 0"];

        let mut dataset = DataSet::default();

        let mut plot_info = read_lines(&cfg, &mut dataset, &lines);

        plot_info.draw_calc_bounds(&dataset);

        assert_ne!(plot_info.y_range, 0.0);
    }
}
