use crate::config::{Config, PlotType};
use crate::draw::PlotInfo;
use crate::scale::{ScaledPoint, TransformType};
use crate::types::DataSet;
use colored::Colorize;

fn draw_axes(config: &Config, plot_info: &mut PlotInfo, rows: &mut [Vec<String>]) {
    let (r, g, b) = config.color_scheme.axis_color();
    for (i, row) in rows.iter_mut().enumerate() {
        let c = if plot_info.draw_y_axis {
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
        row[plot_info.x_axis] = c.truecolor(r, g, b).to_string();
    }

    for i in 0..plot_info.width {
        let c = if plot_info.draw_x_axis {
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
        rows[plot_info.y_axis][i] = c.truecolor(r, g, b).to_string();
    }

    rows[plot_info.y_axis][plot_info.x_axis] = "+".truecolor(r, g, b).to_string();
}

fn print_header(config: &Config, plot_info: &mut PlotInfo, point_counts: bool, columns: u8) {
    if plot_info.log_x {
        print!(
            "    x: log [{} - {}]",
            plot_info.x_min.exp(),
            plot_info.x_max.exp()
        );
    } else {
        print!("    x: [{} - {}]", plot_info.x_min, plot_info.x_max);
    }

    if plot_info.log_y {
        print!(
            "    y: log [{} - {}]",
            plot_info.y_min.exp(),
            plot_info.y_max.exp()
        );
    } else {
        print!("    y: [{} - {}]", plot_info.y_min, plot_info.y_max);
    }

    if !point_counts {
        print!(" -- ");
        let count_key = (0..columns)
            .map(|i| {
                let (r, g, b) = config.color_scheme.series_color(i);
                col_mark(i).to_string().truecolor(r, g, b).to_string()
            })
            .collect::<Vec<_>>()
            .join(", ");
        print!("{}", count_key);
    }
    println!();
}

const COL_MARKS: &[u8; 33] = b"#@*^!~%ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn col_mark(col: u8) -> char {
    if col < COL_MARKS.len() as u8 {
        COL_MARKS[col as usize] as char
    } else {
        '*'
    }
}

fn plot_points(
    config: &Config,
    plot_info: &mut PlotInfo,
    dataset: &DataSet,
    rows: &mut [Vec<String>],
) {
    for c in 0..dataset.columns {
        for r in 0..dataset.rows {
            let p = dataset.points[c as usize][r];
            if p.is_empty() {
                continue;
            }
            let transform = TransformType::new(plot_info.log_x, plot_info.log_y);
            let sp = ScaledPoint::new(p, plot_info, transform);
            let mut mark = col_mark(c);
            if let Some(counters) = &plot_info.counters {
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

            let (r, g, b) = config.color_scheme.series_color(c);
            let mark = mark.to_string().truecolor(r, g, b).to_string();

            rows[sp.1 as usize][sp.0 as usize] = mark.to_string();
        }
    }
}

fn get_rows(plot_info: &mut PlotInfo) -> Vec<Vec<String>> {
    let s = String::from(" ");
    let row = vec![s.clone(); plot_info.width];
    let mut rows = Vec::new();
    for _ in 0..plot_info.height {
        rows.push(row.clone());
    }

    rows
}

pub fn ascii_plot(config: &Config, dataset: &DataSet, plot_info: &mut PlotInfo) {
    let mut rows = get_rows(plot_info);
    if config.axis {
        plot_info.draw_calc_axis_pos();
        draw_axes(config, plot_info, &mut rows);
    }

    print_header(
        config,
        plot_info,
        config.mode == PlotType::Count,
        dataset.columns,
    );
    plot_points(config, plot_info, dataset, &mut rows);

    for row in &rows {
        println!("{}", row.join(""));
    }
}