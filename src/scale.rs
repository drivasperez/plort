use crate::{
    draw::{Plot, PlotInfo},
    types::Point,
};

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

impl ScaledPoint {
    pub fn new_from_plot(point: Point, plot_info: &Plot, transform: TransformType) -> Self {
        let pad = 2;
        let Point(x, y) = point.scale_transform(transform);
        debug_assert!(x >= plot_info.x_min() - f64::EPSILON);
        debug_assert!(x <= plot_info.x_max() + f64::EPSILON);

        let cell_w: f64 = plot_info.x_range() / (plot_info.width() as f64);
        let cell_h: f64 = plot_info.y_range() / (plot_info.height() as f64);

        let ox = ((plot_info.width() as f64 - pad as f64)
            * ((x - plot_info.x_min() + cell_w / 2.0) / plot_info.x_range()))
            as i32;

        let oy = ((plot_info.height() as f64 - pad as f64)
            * ((y - plot_info.y_min() + cell_h / 2.0) / plot_info.y_range()))
            as i32;

        let oy = plot_info.height() as i32 - oy - 1;

        ScaledPoint(ox, oy)
    }

    pub fn new(point: Point, plot_info: &PlotInfo, transform: TransformType) -> Self {
        let pad = 2;
        let Point(x, y) = point.scale_transform(transform);
        debug_assert!(x >= plot_info.x_min - f64::EPSILON);
        debug_assert!(x <= plot_info.x_max + f64::EPSILON);

        let cell_w: f64 = plot_info.x_range / (plot_info.width as f64);
        let cell_h: f64 = plot_info.y_range / (plot_info.height as f64);

        let ox = ((plot_info.width as f64 - pad as f64)
            * ((x - plot_info.x_min + cell_w / 2.0) / plot_info.x_range)) as i32;

        let oy = ((plot_info.height as f64 - pad as f64)
            * ((y - plot_info.y_min + cell_h / 2.0) / plot_info.y_range)) as i32;

        let oy = plot_info.height as i32 - oy - 1;

        ScaledPoint(ox, oy)
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

    fn init_plot_info(plot_info: &mut PlotInfo) -> TransformType {
        plot_info.x_range = plot_info.x_max - plot_info.x_min;
        plot_info.y_range = plot_info.y_max - plot_info.y_min;

        if plot_info.width == 0 {
            plot_info.width = 72;
        }

        if plot_info.height == 0 {
            plot_info.height = 40;
        }

        TransformType::new(plot_info.log_x, plot_info.log_y)
    }

    #[test]
    fn scale_basic() {
        let mut p_i = PlotInfo::default();
        p_i.x_min = 0.0;
        p_i.x_max = 100.0;
        p_i.y_min = 0.0;
        p_i.y_max = 100.0;

        let transform = init_plot_info(&mut p_i);
        let p = Point(0.0, 0.0);
        let sp = ScaledPoint::new(p, &p_i, transform);

        assert_eq!(sp, ScaledPoint(0, p_i.height as i32 - 1));
    }

    #[test]
    fn scale_centered_origin() {
        let mut p_i = PlotInfo::default();
        p_i.x_min = -100.0;
        p_i.x_max = 100.0;
        p_i.y_min = -100.0;
        p_i.y_max = 100.0;

        let transform = init_plot_info(&mut p_i);
        let p = Point(0.0, 0.0);
        let sp = ScaledPoint::new(p, &p_i, transform);

        assert_eq!(
            sp,
            ScaledPoint(p_i.width as i32 / 2 - 1, p_i.height as i32 / 2)
        );
    }

    #[test]
    fn scale_example_regression() {
        let mut p_i = PlotInfo::default();
        p_i.x_min = 1000.0;
        p_i.x_max = 1040.0;
        p_i.y_min = 1000.0;
        p_i.y_max = 1080.0;

        p_i.width = 640;
        p_i.height = 480;

        let transform = init_plot_info(&mut p_i);

        let p0 = Point(1000.0, 992.0);
        let p1 = Point(1040.0, 1066.0);

        let sp0 = ScaledPoint::new(p0, &p_i, transform);
        let sp1 = ScaledPoint::new(p1, &p_i, transform);

        assert_eq!(sp0, ScaledPoint(0, 526));
        assert_eq!(sp1, ScaledPoint(638, 85));
    }

    #[test]
    fn scale_points() {
        let mut p_i = PlotInfo::default();
        p_i.x_min = -100.0;
        p_i.x_max = 100.0;
        p_i.y_min = -100.0;
        p_i.y_max = 100.0;

        let transform = init_plot_info(&mut p_i);
        let p0 = Point(10.0, 20.0);
        let p1 = Point(50.0, 50.0);
        let p2 = Point(-25.0, -10.0);
        let p3 = Point(15.0, 20.0);
        let p4 = Point(-7.0, 8.0);

        let sp0 = ScaledPoint::new(p0, &p_i, transform);
        let sp1 = ScaledPoint::new(p1, &p_i, transform);
        let sp2 = ScaledPoint::new(p2, &p_i, transform);
        let sp3 = ScaledPoint::new(p3, &p_i, transform);
        let sp4 = ScaledPoint::new(p4, &p_i, transform);

        assert_eq!(sp0, ScaledPoint(38, 16));
        assert_eq!(sp1, ScaledPoint(52, 11));
        assert_eq!(sp2, ScaledPoint(26, 22));
        assert_eq!(sp3, ScaledPoint(40, 16));
        assert_eq!(sp4, ScaledPoint(33, 19));
    }

    #[test]
    fn scale_basic_log() {
        let mut p_i = PlotInfo::default();
        p_i.x_min = 0.0;
        p_i.x_max = f64::ln(100.0);
        p_i.y_min = 0.0;
        p_i.y_max = f64::ln(1000.0);

        p_i.log_x = true;
        p_i.log_y = true;

        let transform = init_plot_info(&mut p_i);
        let p = Point(50.0, 100.0);
        let sp = ScaledPoint::new(p, &p_i, transform);

        assert_eq!(sp, ScaledPoint(59, 14));
    }

    #[test]
    fn scale_points_log() {
        let mut p_i = PlotInfo::default();
        p_i.x_min = -100.0;
        p_i.x_max = 100.0;
        p_i.y_min = 0.0;
        p_i.y_max = f64::ln(100.0);

        p_i.log_y = true;

        let transform = init_plot_info(&mut p_i);

        let p0 = Point(10.0, 20.0);
        let p1 = Point(50.0, 50.0);
        let p2 = Point(-25.0, 1.0);
        let p3 = Point(15.0, 20.0);
        let p4 = Point(-7.0, 8.0);

        let sp0 = ScaledPoint::new(p0, &p_i, transform);
        let sp1 = ScaledPoint::new(p1, &p_i, transform);
        let sp2 = ScaledPoint::new(p2, &p_i, transform);
        let sp3 = ScaledPoint::new(p3, &p_i, transform);
        let sp4 = ScaledPoint::new(p4, &p_i, transform);

        assert_eq!(sp0, ScaledPoint(38, 14));
        assert_eq!(sp1, ScaledPoint(52, 7));
        assert_eq!(sp2, ScaledPoint(26, 39));
        assert_eq!(sp3, ScaledPoint(40, 14));
        assert_eq!(sp4, ScaledPoint(33, 22));
    }

    #[test]
    fn scale_out_of_bounds() {
        let mut p_i = PlotInfo::default();
        p_i.x_min = 1000.0;
        p_i.x_max = 1040.0;
        p_i.y_min = 1850.0;
        p_i.y_max = 1925.0;

        p_i.width = 640;
        p_i.height = 480;

        let transform = init_plot_info(&mut p_i);
        let p1 = Point(1000.0, 1850.0);
        let p2 = Point(1040.0, 1924.0);

        let sp1 = ScaledPoint::new(p1, &p_i, transform);
        let sp2 = ScaledPoint::new(p2, &p_i, transform);

        assert_eq!(sp1, ScaledPoint(0, 479));
        assert_eq!(sp2, ScaledPoint(638, 7));
    }
}
