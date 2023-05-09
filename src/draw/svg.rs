use crate::{
    config::PlotType,
    draw::Plot,
    scale::{ScaledPoint, TransformType},
    types::Point,
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
    pub fn get_color(&self, col: usize) -> &str {
        self.colors.get(col).unwrap_or(
            self.colors
                .last()
                .expect("SvgTheme::get_color: no colors defined"),
        )
    }
}

pub fn svg_plot(plot: &Plot, theme: &SvgTheme) {
    print_header(plot.width(), plot.height());
    print_frame(plot.width(), plot.height(), theme);

    if plot.config.axis {
        print_axis(plot, theme);
    }

    let transform = TransformType::new(plot.log_x(), plot.log_y());

    for c in 0..plot.dataset.columns {
        let color = theme.get_color(c);
        let column = &plot.dataset.points[c];

        if plot.config.mode == PlotType::Line {
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

                let sp = ScaledPoint::new_from_plot(*p, plot, transform);
                polyline_point(sp.x(), sp.y());
            }

            if !beginning_line {
                end_polyline(color, theme.line_width);
            }
        } else {
            for p in column {
                let sp = ScaledPoint::new_from_plot(*p, plot, transform);
                if p.is_empty() {
                    continue;
                }

                let point_size = 3.0;
                if let PlotType::Count = plot.config.mode {
                    let counters = &plot.counters();
                    let counter = counters.counters.get(c).unwrap();
                    let count = counter.get(&(sp.0, sp.1)).unwrap_or(&0);
                    let r = if plot.config.log_count {
                        (*count as f64).ln() + point_size
                    } else {
                        *count as f64 + point_size
                    };
                    print_circle(sp.x(), sp.y(), r, color);
                } else {
                    print_circle(sp.x(), sp.y(), point_size, color);
                }
            }
        }

        if plot.config.regression {
            let regression = crate::regression::linear_regression(column, transform);
            if let Some(regression) = regression {
                regression_line(plot, color, regression);
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

fn print_axis(plot: &Plot, theme: &SvgTheme) {
    let tick_width = 3.0 * theme.axis_width;
    let (x_axis, y_axis) = plot.axis_positions();

    if plot.draw_y_axis() {
        // Y-axis
        println!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-dasharray="2.5"/>"#,
            x_axis,
            0,
            x_axis,
            plot.height(),
            theme.axis_color,
            theme.axis_width
        );

        // Ticks
        let mut x0 = x_axis as f64 - tick_width;
        let x1 = x_axis as f64 + tick_width;
        if x0 > plot.width() as f64 {
            x0 = 0.0;
        }

        let y_to = scale_tick(plot.height(), plot.y_range());

        let mut hy = y_axis as f64 + y_to;
        while hy < plot.height() as f64 {
            println!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" />"#,
                x0, hy, x1, hy, theme.axis_color
            );
            hy += y_to;
        }
        let mut hy = y_axis as f64 - y_to;
        while hy > 0.0 {
            println!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" />"#,
                x0, hy, x1, hy, theme.axis_color
            );
            hy -= y_to;
        }
    }

    if plot.draw_x_axis() {
        // X-axis
        println!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-dasharray="2.5" />"#,
            0,
            y_axis,
            plot.width(),
            y_axis,
            theme.axis_color,
            theme.axis_width
        );

        // Ticks
        let mut y0 = y_axis as f64 - tick_width;
        let y1 = y_axis as f64 + tick_width;
        if y0 > plot.height() as f64 {
            y0 = 0.0;
        }

        let x_to = scale_tick(plot.width(), plot.x_range());

        let mut wx = x_axis as f64 + x_to;
        while wx < plot.width() as f64 {
            println!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" />"#,
                wx, y0, wx, y1, theme.axis_color
            );
            wx += x_to;
        }
        let mut wx = x_axis as f64 - x_to;
        while wx > 0.0 {
            println!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" />"#,
                wx, y0, wx, y1, theme.axis_color
            );
            wx -= x_to;
        }
    }
}

fn regression_line(plot: &Plot, color: &str, regression: (f64, f64)) {
    let (slope, intercept) = regression;

    let p0 = Point(plot.x_min(), intercept + slope * plot.x_min());
    let p1 = Point(plot.x_max(), intercept + slope * plot.x_max());

    // Already scaled, so no need to scale again. Just need to create a ScaledPoint.
    let p0 = ScaledPoint::new_from_plot(p0, plot, crate::scale::TransformType::None);
    let p1 = ScaledPoint::new_from_plot(p1, plot, crate::scale::TransformType::None);

    println!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-dasharray="5" />"#,
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
