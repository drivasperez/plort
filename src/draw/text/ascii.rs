use crate::config::PlotType;
use crate::draw::Plot;
use crate::scale::{ScaledPoint, TransformType};
use colored::Colorize;

pub struct AsciiPlot<'a> {
    plot: &'a Plot<'a>,
    rows: Vec<Vec<String>>,
}

impl<'a> AsciiPlot<'a> {
    pub fn new(plot: &'a Plot<'a>) -> Self {
        let s = String::from(" ");
        let row = vec![s; plot.width()];
        let mut rows = Vec::new();
        for _ in 0..plot.height() {
            rows.push(row.clone());
        }

        Self { plot, rows }
    }

    fn print_header(&mut self) {
        let point_counts = self.plot.config.mode == PlotType::Count;
        let columns = self.plot.dataset.columns;
        if self.plot.config.log_x {
            print!(
                "    x: log [{} - {}]",
                self.plot.x_min().exp(),
                self.plot.x_max().exp()
            );
        } else {
            print!("    x: [{} - {}]", self.plot.x_min(), self.plot.x_max());
        }

        if self.plot.log_y() {
            print!(
                "    y: log [{} - {}]",
                self.plot.y_min().exp(),
                self.plot.y_max().exp()
            );
        } else {
            print!("    y: [{} - {}]", self.plot.y_min(), self.plot.y_max());
        }

        if !point_counts {
            print!(" -- ");
            let count_key = (0..columns)
                .map(|i| {
                    let (r, g, b) = self.plot.config.color_scheme.series_color(i);
                    col_mark(i).to_string().truecolor(r, g, b).to_string()
                })
                .collect::<Vec<_>>()
                .join(", ");
            print!("{}", count_key);
        }
        println!();
    }

    fn draw_axes(&mut self) {
        let (x_axis, y_axis) = self.plot.axis_positions();

        let (r, g, b) = self.plot.config.color_scheme.axis_color();
        for (i, row) in self.rows.iter_mut().enumerate() {
            let c = if self.plot.draw_y_axis() {
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
            row[x_axis] = c.truecolor(r, g, b).to_string();
        }

        for i in 0..self.plot.width() {
            let c = if self.plot.draw_x_axis() {
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
            self.rows[y_axis][i] = c.truecolor(r, g, b).to_string();
        }

        self.rows[y_axis][x_axis] = "+".truecolor(r, g, b).to_string();
    }

    fn plot_points(&mut self) {
        for c in 0..self.plot.dataset.columns {
            for r in 0..self.plot.dataset.rows {
                let p = self.plot.dataset.points[c as usize][r];
                if p.is_empty() {
                    continue;
                }
                let transform = TransformType::new(self.plot.log_x(), self.plot.log_y());
                let sp = ScaledPoint::new_from_plot(p, self.plot, transform);
                let mut mark = col_mark(c);
                if let PlotType::Count = self.plot.config.mode {
                    let counters = &mut self.plot.counters();
                    let count = counters.counters[c as usize].get(&(sp.0, sp.1));
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

                let (r, g, b) = self.plot.config.color_scheme.series_color(c);
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

pub fn ascii_plot(plot: &Plot) {
    let mut graph = AsciiPlot::new(plot);
    if plot.config.axis {
        graph.draw_axes();
    }

    graph.print_header();
    graph.plot_points();

    graph.print_graph();
}
