use crate::{draw::Plot, types::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScaledPoint(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformType {
    None,
    LogX,
    LogY,
    LogXY,
}

impl TransformType {
    pub fn new(log_x: bool, log_y: bool) -> Self {
        match (log_x, log_y) {
            (true, true) => TransformType::LogXY,
            (true, false) => TransformType::LogX,
            (false, true) => TransformType::LogY,
            (false, false) => TransformType::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub width: usize,
    pub height: usize,
}

impl ScaledPoint {
    pub fn new_from_bounds(
        point: Point,
        transform: TransformType,
        Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            width,
            height,
        }: Bounds,
    ) -> Self {
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;
        let pad = 2;
        let Point(x, y) = point.scale_transform(transform);
        debug_assert!(x >= x_min - f64::EPSILON);
        debug_assert!(x <= x_max + f64::EPSILON);

        let cell_w: f64 = x_range / (width as f64);
        let cell_h: f64 = y_range / (height as f64);

        let ox = ((width as f64 - pad as f64) * ((x - x_min + cell_w / 2.0) / x_range)) as i32;

        let oy = ((height as f64 - pad as f64) * ((y - y_min + cell_h / 2.0) / y_range)) as i32;

        let oy = height as i32 - oy - 1;

        ScaledPoint(ox, oy)
    }

    pub fn new_from_plot(point: Point, plot_info: &Plot, transform: TransformType) -> Self {
        Self::new_from_bounds(
            point,
            transform,
            Bounds {
                x_min: plot_info.x_min(),
                x_max: plot_info.x_max(),
                y_min: plot_info.y_min(),
                y_max: plot_info.y_max(),
                width: plot_info.width(),
                height: plot_info.height(),
            },
        )
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn get_transform() {
        let transform = TransformType::new(true, true);
        assert_eq!(transform, TransformType::LogXY);
        let transform = TransformType::new(true, false);
        assert_eq!(transform, TransformType::LogX);
        let transform = TransformType::new(false, true);
        assert_eq!(transform, TransformType::LogY);
        let transform = TransformType::new(false, false);
        assert_eq!(transform, TransformType::None);
    }

    #[test]
    fn transform() {
        let p = Point(10.0, 100.0);
        let out_none = p.scale_transform(TransformType::None);
        assert_eq!(out_none, Point(10.0, 100.0));
        let out_logx = p.scale_transform(TransformType::LogX);
        assert_eq!(out_logx, Point(f64::ln(10.0), 100.0));
        let out_logy = p.scale_transform(TransformType::LogY);
        assert_eq!(out_logy, Point(10.0, f64::ln(100.0)));
        let out_logxy = p.scale_transform(TransformType::LogXY);
        assert_eq!(out_logxy, Point(f64::ln(10.0), f64::ln(100.0)));
    }

    #[test]
    fn scale_basic() {
        let bounds = Bounds {
            x_min: 0.0,
            x_max: 100.0,
            y_min: 0.0,
            y_max: 100.0,
            width: 72,
            height: 40,
        };

        let transform = TransformType::new(false, false);
        let p = Point(0.0, 0.0);
        let sp = ScaledPoint::new_from_bounds(p, transform, bounds.clone());

        assert_eq!(sp, ScaledPoint(0, bounds.height as i32 - 1));
    }

    #[test]
    fn scale_centered_origin() {
        let x_min = -100.0;
        let x_max = 100.0;
        let y_min = -100.0;
        let y_max = 100.0;
        let width = 72;
        let height = 40;

        let bounds = Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            width,
            height,
        };

        let transform = TransformType::new(false, false);
        let p = Point(0.0, 0.0);
        let sp = ScaledPoint::new_from_bounds(p, transform, bounds);

        assert_eq!(sp, ScaledPoint(width as i32 / 2 - 1, height as i32 / 2));
    }

    #[test]
    fn scale_example_regression() {
        let x_min = 1000.0;
        let x_max = 1040.0;
        let y_min = 1000.0;
        let y_max = 1080.0;

        let width = 640;
        let height = 480;

        let bounds = Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            width,
            height,
        };

        let transform = TransformType::new(false, false);
        let p0 = Point(1000.0, 992.0);
        let p1 = Point(1040.0, 1066.0);

        let sp0 = ScaledPoint::new_from_bounds(p0, transform, bounds.clone());
        let sp1 = ScaledPoint::new_from_bounds(p1, transform, bounds);

        assert_eq!(sp0, ScaledPoint(0, 526));
        assert_eq!(sp1, ScaledPoint(638, 85));
    }

    #[test]
    fn scale_points() {
        let x_min = -100.0;
        let x_max = 100.0;
        let y_min = -100.0;
        let y_max = 100.0;

        let bounds = Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            width: 72,
            height: 40,
        };

        let transform = TransformType::new(false, false);
        let p0 = Point(10.0, 20.0);
        let p1 = Point(50.0, 50.0);
        let p2 = Point(-25.0, -10.0);
        let p3 = Point(15.0, 20.0);
        let p4 = Point(-7.0, 8.0);

        let sp0 = ScaledPoint::new_from_bounds(p0, transform, bounds.clone());
        let sp1 = ScaledPoint::new_from_bounds(p1, transform, bounds.clone());
        let sp2 = ScaledPoint::new_from_bounds(p2, transform, bounds.clone());
        let sp3 = ScaledPoint::new_from_bounds(p3, transform, bounds.clone());
        let sp4 = ScaledPoint::new_from_bounds(p4, transform, bounds);

        assert_eq!(sp0, ScaledPoint(38, 16));
        assert_eq!(sp1, ScaledPoint(52, 11));
        assert_eq!(sp2, ScaledPoint(26, 22));
        assert_eq!(sp3, ScaledPoint(40, 16));
        assert_eq!(sp4, ScaledPoint(33, 19));
    }

    #[test]
    fn scale_basic_log() {
        let x_min = 0.0;
        let x_max = f64::ln(100.0);
        let y_min = 0.0;
        let y_max = f64::ln(1000.0);

        let bounds = Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            width: 72,
            height: 40,
        };

        let transform = TransformType::new(true, true);

        let p = Point(50.0, 100.0);
        let sp = ScaledPoint::new_from_bounds(p, transform, bounds);

        assert_eq!(sp, ScaledPoint(59, 14));
    }

    #[test]
    fn scale_points_log() {
        let x_min = -100.0;
        let x_max = 100.0;
        let y_min = 0.0;
        let y_max = f64::ln(100.0);

        let bounds = Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            width: 72,
            height: 40,
        };

        let transform = TransformType::new(false, true);

        let p0 = Point(10.0, 20.0);
        let p1 = Point(50.0, 50.0);
        let p2 = Point(-25.0, 1.0);
        let p3 = Point(15.0, 20.0);
        let p4 = Point(-7.0, 8.0);

        let sp0 = ScaledPoint::new_from_bounds(p0, transform, bounds.clone());
        let sp1 = ScaledPoint::new_from_bounds(p1, transform, bounds.clone());
        let sp2 = ScaledPoint::new_from_bounds(p2, transform, bounds.clone());
        let sp3 = ScaledPoint::new_from_bounds(p3, transform, bounds.clone());
        let sp4 = ScaledPoint::new_from_bounds(p4, transform, bounds);

        assert_eq!(sp0, ScaledPoint(38, 14));
        assert_eq!(sp1, ScaledPoint(52, 7));
        assert_eq!(sp2, ScaledPoint(26, 39));
        assert_eq!(sp3, ScaledPoint(40, 14));
        assert_eq!(sp4, ScaledPoint(33, 22));
    }

    #[test]
    fn scale_out_of_bounds() {
        let x_min = 1000.0;
        let x_max = 1040.0;
        let y_min = 1850.0;
        let y_max = 1925.0;

        let bounds = Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            width: 640,
            height: 480,
        };

        let transform = TransformType::new(false, false);
        let p1 = Point(1000.0, 1850.0);
        let p2 = Point(1040.0, 1924.0);

        let sp1 = ScaledPoint::new_from_bounds(p1, transform, bounds.clone());
        let sp2 = ScaledPoint::new_from_bounds(p2, transform, bounds);

        assert_eq!(sp1, ScaledPoint(0, 479));
        assert_eq!(sp2, ScaledPoint(638, 7));
    }
}
