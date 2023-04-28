use crate::{draw::PlotInfo, types::Point};
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
    pub fn new(point: Point, plot_info: &PlotInfo, transform: TransformType) -> Self {
        let pad = 2;
        let (x, y) = scale_transform(&point, transform);
        debug_assert!(x >= plot_info.x_min - f64::EPSILON);
        debug_assert!(x <= plot_info.x_max + f64::EPSILON);

        let cell_w: f64 = plot_info.x_range / (plot_info.width as f64);
        let cell_h: f64 = plot_info.y_range / (plot_info.height as f64);

        let ox = ((plot_info.width - pad) as f64
            * ((x - plot_info.x_min + cell_w / 2.0) / plot_info.x_range)) as i32;

        let oy = ((plot_info.height - pad) as f64
            * ((y - plot_info.y_min + cell_h / 2.0) / plot_info.y_range)) as i32;

        let oy = plot_info.height as i32 - oy - 1;

        ScaledPoint(ox, oy)
    }
}

fn scale_transform(point: &Point, transform: TransformType) -> (f64, f64) {
    match transform {
        TransformType::None => (point.0, point.1),
        TransformType::LogX => (point.0.log10(), point.1),
        TransformType::LogY => (point.0, point.1.log10()),
        TransformType::LogXY => (point.0.log10(), point.1.log10()),
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
        let out_none = scale_transform(&p, TransformType::None);
        assert_eq!(out_none, (10.0, 100.0));
        let out_logx = scale_transform(&p, TransformType::LogX);
        assert_eq!(out_logx, (1.0, 100.0));
        let out_logy = scale_transform(&p, TransformType::LogY);
        assert_eq!(out_logy, (10.0, 2.0));
        let out_logxy = scale_transform(&p, TransformType::LogXY);
        assert_eq!(out_logxy, (1.0, 2.0));
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
}
