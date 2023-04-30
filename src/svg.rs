use crate::{
    config::{Config, PlotType},
    draw::PlotInfo,
    scale::{ScaledPoint, TransformType},
    types::{DataSet, Point},
};

pub struct SvgTheme {
    pub bg_color: String,
    pub border_color: String,
    pub axis_color: String,
    pub colors: Vec<String>,
    pub line_width: f64,
    pub axis_width: f64,
    pub border_width: f64,
}

impl SvgTheme {
    pub fn get_color(&self, col: u8) -> &str {
        self.colors.get(col as usize).unwrap_or(
            &self
                .colors
                .last()
                .expect("SvgTheme::get_color: no colors defined"),
        )
    }
}

pub fn svg_plot(config: &Config, plot_info: &mut PlotInfo, dataset: &DataSet, theme: &SvgTheme) {
    print_header(plot_info.width, plot_info.height);
    print_frame(plot_info.width, plot_info.height, theme);

    if config.axis {
        plot_info.draw_calc_axis_pos();
        print_axis(plot_info, theme);
    }

    let transform = TransformType::new(plot_info.log_x, plot_info.log_y);

    for c in 0..dataset.columns {
        let color = theme.get_color(c);
        let column = &dataset.points[c as usize];

        if config.mode == PlotType::Line {
            let mut beginning_line = true;
            for p in column {
                if p.is_empty() {
                    if !beginning_line {
                        end_polyline(color, theme.line_width);
                    }
                    beginning_line = true;
                    continue;
                }

                if beginning_line {
                    beginning_line = false;
                    begin_polyline();
                }

                let sp = ScaledPoint::new(*p, &plot_info, transform);
                polyline_point(sp.x(), sp.y());
            }

            if !beginning_line {
                end_polyline(color, theme.line_width);
            }
        } else {
            for p in column {
                let sp = ScaledPoint::new(*p, &plot_info, transform);
                if p.is_empty() {
                    continue;
                }

                let point_size = 3.0;
                if let Some(counters) = &plot_info.counters {
                    let counter = counters.get(c as usize).unwrap();
                    let count = counter.get(&(sp.0, sp.1)).unwrap_or(&0);
                    let r = if config.log_count {
                        (*count as f64).ln() + point_size
                    } else {
                        *count as f64 + point_size
                    };
                    print_circle(sp.x(), sp.y(), r, color);
                } else {
                    print_circle(sp.x(), sp.y(), point_size as f64, color);
                }
            }
        }

        if config.regression {
            let regression = crate::regression::linear_regression(column, transform);
            if let Some(regression) = regression {
                regression_line(plot_info, color, regression);
            }
        }
    }

    end_svg();
}

fn print_header(width: usize, height: usize) {
    println!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" version="1.1">"#,
        width, height
    );
}

fn print_frame(width: usize, height: usize, theme: &SvgTheme) {
    println!(
        r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
        width, height, theme.bg_color, theme.border_color, theme.border_width
    );
}

fn begin_polyline() {
    println!(r#"<polyline points=""#);
}

fn polyline_point(x: i32, y: i32) {
    print!("{},{} ", x, y);
}

fn end_polyline(color: &str, line_width: f64) {
    println!(
        r#"" fill="none" stroke="{}" stroke-width="{}"/>"#,
        color, line_width
    );
}

fn print_circle(x: i32, y: i32, r: f64, color: &str) {
    println!(
        r#"<circle cx="{}" cy="{}" r="{}" stroke="{}"/>"#,
        x, y, r, color
    );
}

fn print_axis(plot_info: &PlotInfo, theme: &SvgTheme) {
    let tick_width = 3.0 * theme.axis_width;

    if plot_info.draw_y_axis {
        // Y-axis
        println!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-dasharray="2.5"/n>"#,
            plot_info.x_axis,
            0,
            plot_info.x_axis,
            plot_info.height,
            theme.axis_color,
            theme.axis_width
        );

        // Ticks
        let mut x0 = plot_info.x_axis as f64 - tick_width;
        let x1 = plot_info.x_axis as f64 + tick_width;
        if x0 > plot_info.width as f64 {
            x0 = 0.0;
        }

        let y_to = scale_tick(plot_info.height, plot_info.y_range);

        let mut hy = plot_info.y_axis as f64 + y_to;
        while hy < plot_info.height as f64 {
            println!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" /n>"#,
                x0, hy, x1, hy, theme.axis_color
            );
            hy += y_to;
        }
        let mut hy = plot_info.y_axis as f64 - y_to;
        while hy > 0.0 {
            println!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" /n>"#,
                x0, hy, x1, hy, theme.axis_color
            );
            hy -= y_to;
        }
    }

    if plot_info.draw_x_axis {
        // X-axis
        println!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-dasharray="2.5" /n>"#,
            0,
            plot_info.y_axis,
            plot_info.width,
            plot_info.y_axis,
            theme.axis_color,
            theme.axis_width
        );

        // Ticks
        let mut y0 = plot_info.y_axis as f64 - tick_width;
        let y1 = plot_info.y_axis as f64 + tick_width;
        if y0 > plot_info.height as f64 {
            y0 = 0.0;
        }

        let x_to = scale_tick(plot_info.width, plot_info.x_range);

        let mut wx = plot_info.x_axis as f64 + x_to;
        while wx < plot_info.width as f64 {
            println!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" /n>"#,
                wx, y0, wx, y1, theme.axis_color
            );
            wx += x_to;
        }
        let mut wx = plot_info.x_axis as f64 - x_to;
        while wx > 0.0 {
            println!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" /n>"#,
                wx, y0, wx, y1, theme.axis_color
            );
            wx -= x_to;
        }
    }
}

fn regression_line(plot_info: &PlotInfo, color: &str, regression: (f64, f64)) {
    let (slope, intercept) = regression;

    let p0 = Point(plot_info.x_min, intercept + slope * plot_info.x_min);
    let p1 = Point(plot_info.x_max, intercept + slope * plot_info.x_max);

    // Already scaled, so no need to scale again. Just need to create a ScaledPoint.
    let p0 = ScaledPoint::new(p0, plot_info, crate::scale::TransformType::None);
    let p1 = ScaledPoint::new(p1, plot_info, crate::scale::TransformType::None);

    println!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-dasharray="5" /n>"#,
        p0.x(),
        p0.y(),
        p1.x(),
        p1.y(),
        color,
        2
    );
}

fn end_svg() {
    println!("</svg>");
}

fn scale_tick(width: usize, range: f64) -> f64 {
    let rounded = range.log10().ceil().powf(10.0);
    let div = if range < rounded / 2.0 { 20.0 } else { 10.0 };
    let step = rounded / div;

    width as f64 * (step / range)
}
