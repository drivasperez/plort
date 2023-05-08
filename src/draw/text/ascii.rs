use crate::config::{Config, PlotType};
use crate::draw::PlotInfo;
use crate::scale::{ScaledPoint, TransformType};
use crate::types::DataSet;
use colored::Colorize;

pub struct AsciiPlot<'a> {
    config: &'a Config,
    dataset: &'a DataSet,
    plot_info: &'a mut PlotInfo,
    rows: Vec<Vec<String>>,
}

impl<'a> AsciiPlot<'a> {
    pub fn new(config: &'a Config, dataset: &'a DataSet, plot_info: &'a mut PlotInfo) -> Self {
        let s = String::from(" ");
        let row = vec![s; plot_info.width];
        let mut rows = Vec::new();
        for _ in 0..plot_info.height {
            rows.push(row.clone());
        }

        Self {
            config,
            dataset,
            plot_info,
            rows,
        }
    }

    fn print_header(&mut self) {
        let point_counts = self.config.mode == PlotType::Count;
        let columns = self.dataset.columns;
        if self.plot_info.log_x {
            print!(
                "    x: log [{} - {}]",
                self.plot_info.x_min.exp(),
                self.plot_info.x_max.exp()
            );
        } else {
            print!(
                "    x: [{} - {}]",
                self.plot_info.x_min, self.plot_info.x_max
            );
        }

        if self.plot_info.log_y {
            print!(
                "    y: log [{} - {}]",
                self.plot_info.y_min.exp(),
                self.plot_info.y_max.exp()
            );
        } else {
            print!(
                "    y: [{} - {}]",
                self.plot_info.y_min, self.plot_info.y_max
            );
        }

        if !point_counts {
            print!(" -- ");
            let count_key = (0..columns)
                .map(|i| {
                    let (r, g, b) = self.config.color_scheme.series_color(i);
                    col_mark(i).to_string().truecolor(r, g, b).to_string()
                })
                .collect::<Vec<_>>()
                .join(", ");
            print!("{}", count_key);
        }
        println!();
    }

    fn draw_axes(&mut self) {
        self.plot_info.calc_axis_pos();

        let (r, g, b) = self.config.color_scheme.axis_color();
        for (i, row) in self.rows.iter_mut().enumerate() {
            let c = if self.plot_info.draw_y_axis {
                if i % 5 == 0 {
                    "+"
                } else {
                    "|"
                }
            } else if i % 5 == 0 {
                "."
            } else {
                " "
            };
            row[self.plot_info.x_axis] = c.truecolor(r, g, b).to_string();
        }

        for i in 0..self.plot_info.width {
            let c = if self.plot_info.draw_x_axis {
                if i % 5 == 0 {
                    "+"
                } else {
                    "─"
                }
            } else if i % 5 == 0 {
                "."
            } else {
                " "
            };
            // rows[plot_info.y_axis][i] = c.truecolor(211, 219, 216).to_string();
            self.rows[self.plot_info.y_axis][i] = c.truecolor(r, g, b).to_string();
        }

        self.rows[self.plot_info.y_axis][self.plot_info.x_axis] =
            "+".truecolor(r, g, b).to_string();
    }

    fn plot_points(&mut self) {
        for c in 0..self.dataset.columns {
            for r in 0..self.dataset.rows {
                let p = self.dataset.points[c as usize][r];
                if p.is_empty() {
                    continue;
                }
                let transform = TransformType::new(self.plot_info.log_x, self.plot_info.log_y);
                let sp = ScaledPoint::new(p, self.plot_info, transform);
                let mut mark = col_mark(c);
                if let Some(counters) = &self.plot_info.counters {
                    let count = counters[c as usize].get(&(sp.0, sp.1));
                    if let Some(&count) = count {
                        if count < 10 {
                            // This is not a good idea with unicode, but it works for ASCII
                            mark = (b'0' + count as u8) as char;
                        } else if count < 36 {
                            mark = (b'a' + count as u8) as char;
                        } else {
                            mark = '#';
                        }
                    }
                }

                let (r, g, b) = self.config.color_scheme.series_color(c);
                let mark = mark.to_string().truecolor(r, g, b).to_string();

                self.rows[sp.1 as usize][sp.0 as usize] = mark.to_string();
            }
        }
    }

    pub fn print_graph(&self) {
        for row in &self.rows {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }
}

const COL_MARKS: &[u8; 33] = b"#@*^!~%ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn col_mark(col: usize) -> char {
    if col < COL_MARKS.len() {
        COL_MARKS[col] as char
    } else {
        '*'
    }
}

pub fn ascii_plot(config: &Config, dataset: &DataSet, plot_info: &mut PlotInfo) {
    let mut graph = AsciiPlot::new(config, dataset, plot_info);
    if config.axis {
        graph.draw_axes();
    }

    graph.print_header();
    graph.plot_points();

    graph.print_graph();
}
