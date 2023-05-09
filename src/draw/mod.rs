use std::collections::HashMap;

use crate::config::{Config, OutputType};
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

pub struct Plot<'a> {
    dataset: &'a DataSet,
    config: &'a Config,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl<'a> Plot<'a> {
    pub fn new(dataset: &'a DataSet, config: &'a Config) -> anyhow::Result<Self> {
        let mut min_point = Point(f64::MAX, f64::MAX);
        let mut max_point = Point(f64::MIN, f64::MIN);

        for col in 0..dataset.columns {
            for row in 0..dataset.rows {
                let point = dataset.points[col as usize][row];
                if point.is_empty() {
                    continue;
                }

                if config.log_x && point.0 <= 0.0 {
                    anyhow::bail!("Log scale requires positive X values");
                }

                if config.log_y && point.1 <= 0.0 {
                    anyhow::bail!("Log scale requires positive Y values");
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

        let transform = TransformType::new(config.log_x, config.log_y);
        let min_point = min_point.scale_transform(transform);
        let mut max_point = max_point.scale_transform(transform);

        if min_point.0 == max_point.0 {
            max_point.0 += 1.0;
        }

        if min_point.1 == max_point.1 {
            max_point.1 += 1.0;
        }

        let mut x_min = min_point.0;
        let mut x_max = max_point.0;
        let mut y_min = min_point.1;
        let mut y_max = max_point.1;

        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        let crosses_x_axis = x_min <= 0.0 && x_max >= 0.0;
        let crosses_y_axis = y_min <= 0.0 && y_max >= 0.0;

        // If the data does not cross the x or y axis, we can
        // clamp the plot's axis to zero.
        if !crosses_x_axis {
            if 0.0 < x_min && 0.0 > x_min - x_range * CROSS_PAD {
                x_min = 0.0;
            } else if 0.0 > x_max && 0.0 < x_max + x_range * CROSS_PAD {
                x_max = 0.0;
            }
        }

        if !crosses_y_axis {
            if 0.0 < y_min && 0.0 > y_min - y_range * CROSS_PAD {
                y_min = 0.0;
            } else if 0.0 > y_max && 0.0 < y_max + y_range * CROSS_PAD {
                y_max = 0.0;
            }
        }

        if x_min == f64::MAX || y_min == f64::MAX {
            anyhow::bail!("No data to plot");
        }

        if x_min == x_max || y_min == y_max {
            anyhow::bail!("Insufficient range of data");
        }

        Ok(Self {
            dataset,
            config,
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }

    pub fn x_min(&self) -> f64 {
        self.x_min
    }

    pub fn x_max(&self) -> f64 {
        self.x_max
    }

    pub fn y_min(&self) -> f64 {
        self.y_min
    }

    pub fn y_max(&self) -> f64 {
        self.y_max
    }

    pub fn height(&self) -> usize {
        self.config.dimensions.height
    }

    pub fn width(&self) -> usize {
        self.config.dimensions.width
    }

    pub fn log_x(&self) -> bool {
        self.config.log_x
    }

    pub fn log_y(&self) -> bool {
        self.config.log_y
    }

    pub fn x_range(&self) -> f64 {
        self.x_max - self.x_min
    }

    pub fn y_range(&self) -> f64 {
        self.y_max - self.y_min
    }

    fn draw_x_axis(&self) -> bool {
        0.0 >= self.y_min && 0.0 <= self.y_max
    }

    fn draw_y_axis(&self) -> bool {
        0.0 >= self.x_min && 0.0 <= self.x_max
    }

    pub fn axis_positions(&self) -> (usize, usize) {
        let mut origin = Point(0.0, 0.0);
        if !self.draw_y_axis() {
            if 0.0 < self.x_min {
                origin.0 = self.x_min;
            } else {
                origin.0 = self.x_max;
            }
        }

        if !self.draw_x_axis() {
            if 0.0 < self.y_min {
                origin.1 = self.y_min;
            } else {
                origin.1 = self.y_max;
            }
        }

        let sp = ScaledPoint::new_from_plot(
            origin,
            self,
            TransformType::new(self.config.log_x, self.config.log_y),
        );
        // XXX Is this safe? SP are signed, but x_axis and y_axis are unsigned.
        (sp.0 as usize, sp.1 as usize)
    }

    pub fn counters(&self) -> Counters {
        let mut counters = Vec::new();
        for col in 0..self.dataset.columns {
            let points = &self.dataset.points[col];
            let transform = TransformType::new(self.config.log_x, self.config.log_y);
            let mut counts = HashMap::new();

            for point in points.iter().take(self.dataset.rows) {
                if point.0.is_nan() || point.1.is_nan() {
                    continue;
                }

                let scaled_point = ScaledPoint::new_from_plot(*point, self, transform);
                let x = scaled_point.0;
                let y = scaled_point.1;

                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;
            }

            counters.push(counts);
        }

        Counters { counters }
    }
}

pub struct Counters {
    counters: Vec<HashMap<(i32, i32), u32>>,
}

pub fn draw(config: &Config, dataset: &DataSet) -> anyhow::Result<()> {
    let plot = Plot::new(dataset, config)?;

    match config.output_type {
        OutputType::Ascii => ascii_plot(&plot),
        OutputType::Braille => braille_plot(&plot),
        OutputType::Svg => {
            // TODO: Configurable theme
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

            svg_plot(&plot, &theme);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::{process_line, ProcessLineResult};

    fn read_lines<'a>(cfg: &'a Config, dataset: &'a mut DataSet, lines: &[&str]) -> Plot<'a> {
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(ProcessLineResult::Ok, process_line(cfg, dataset, line, i));
        }
        Plot::new(dataset, cfg).unwrap()
    }

    #[test]
    fn bounds_basic() {
        let cfg = Config::default();
        let mut dataset = DataSet::default();
        let lines = ["1 2 3", "4 5 6", "7 8 9"];

        let plot = read_lines(&cfg, &mut dataset, &lines);

        dbg!(&plot.x_min);
        dbg!(&plot.x_max);
        dbg!(&plot.y_min);
        dbg!(&plot.y_max);

        assert_eq!(plot.x_min, 0.0);
        assert_eq!(plot.x_max, 2.0);
        assert_eq!(plot.y_min, 0.0);
        assert_eq!(plot.y_max, 9.0);
    }

    #[test]
    fn bounds_empty_cells() {
        let cfg = Config::default();
        let mut dataset = DataSet::default();
        let lines = ["  2 3", "4   6", "7 8  "];

        let plot = read_lines(&cfg, &mut dataset, &lines);

        dbg!(&plot.x_min);
        dbg!(&plot.x_max);
        dbg!(&plot.y_min);
        dbg!(&plot.y_max);

        assert_eq!(plot.x_min, 0.0);
        assert_eq!(plot.x_max, 2.0);
        assert_eq!(plot.y_min, 0.0);
        assert_eq!(plot.y_max, 8.0);
    }

    #[test]
    fn bounds_negative_quadrant() {
        let mut cfg = Config::default();
        cfg.x_column = true;
        let mut dataset = DataSet::default();
        let lines = ["-5 -50", "-3 -30", "-1 -10"];

        let plot = read_lines(&cfg, &mut dataset, &lines);

        dbg!(&plot.x_min);
        dbg!(&plot.x_max);
        dbg!(&plot.y_min);
        dbg!(&plot.y_max);

        assert_eq!(plot.x_min, -5.0);
        assert_eq!(plot.x_max, 0.0);
        assert_eq!(plot.y_min, -50.0);
        assert_eq!(plot.y_max, 0.0);
    }

    #[test]
    fn bounds_4q() {
        let mut cfg = Config::default();
        cfg.x_column = true;
        let mut dataset = DataSet::default();
        let lines = ["-5 -50", "-49 49", "0 0", "10 10"];

        let plot = read_lines(&cfg, &mut dataset, &lines);

        dbg!(&plot.x_min);
        dbg!(&plot.x_max);
        dbg!(&plot.y_min);
        dbg!(&plot.y_max);

        // In the original test it's -50 but it seems like it should be -49?
        assert_eq!(plot.x_min, -49.0);
        assert_eq!(plot.x_max, 10.0);
        assert_eq!(plot.y_min, -50.0);
        assert_eq!(plot.y_max, 49.0);
    }

    #[test]
    fn bounds_diff_quadrants_for_columns() {
        let mut cfg = Config::default();
        cfg.x_column = true;
        let mut dataset = DataSet::default();
        let lines = ["-50 -50 50", "-49 -49 49", "-48 -48 48", "-47 -47 47"];

        let plot = read_lines(&cfg, &mut dataset, &lines);

        dbg!(&plot.x_min);
        dbg!(&plot.x_max);
        dbg!(&plot.y_min);
        dbg!(&plot.y_max);

        assert_eq!(plot.x_min, -50.0);
        assert_eq!(plot.x_max, -47.0);
        assert_eq!(plot.y_min, -50.0);
        assert_eq!(plot.y_max, 50.0);
    }

    #[test]
    fn bounds_too_distant_to_touch_axis() {
        let mut cfg = Config::default();
        cfg.x_column = true;
        let mut dataset = DataSet::default();
        let lines = ["-5000 -5000", "-4900 -4900", "-4800 -4800"];

        let plot = read_lines(&cfg, &mut dataset, &lines);

        dbg!(&plot.x_min);
        dbg!(&plot.x_max);
        dbg!(&plot.y_min);
        dbg!(&plot.y_max);

        assert_eq!(plot.x_min, -5000.0);
        assert_eq!(plot.x_max, -4800.0);
        assert_eq!(plot.y_min, -5000.0);
        assert_eq!(plot.y_max, -4800.0);
    }

    #[test]
    fn bounds_log_basic() {
        let mut cfg = Config::default();
        cfg.log_y = true;

        let lines = ["1214", "358", "316", "187", "186", "93", "63", "11"];

        let mut dataset = DataSet::default();

        let plot_info = read_lines(&cfg, &mut dataset, &lines);

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

        let plot_info = read_lines(&cfg, &mut dataset, &lines);

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

        let plot_info = read_lines(&cfg, &mut dataset, &lines);

        assert_ne!(plot_info.y_range(), 0.0);
    }

    #[test]
    fn reject_y_range_of_zero() {
        let cfg = Config::default();

        let lines = ["0 0", "0 0"];

        let mut dataset = DataSet::default();

        let mut plot_info = read_lines(&cfg, &mut dataset, &lines);

        assert_ne!(plot_info.y_range(), 0.0);
    }
}
