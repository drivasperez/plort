use crate::scale::TransformType;
use crate::types::Point;

fn calc_denom(points: &[Point], transform: TransformType, mx: f64) -> f64 {
    let mut sum = 0.0;
    for point in points {
        let p = point.scale_transform(transform);
        sum += (p.x() - mx).powi(2);
    }

    sum
}

fn calc_numerator(points: &[Point], transform: TransformType, mx: f64, my: f64) -> f64 {
    let mut sum = 0.0;
    for point in points {
        let p = point.scale_transform(transform);
        sum += (p.x() - mx) * (p.y() - my);
    }

    sum
}

fn calc_means(points: &[Point], transform: TransformType) -> Option<(f64, f64)> {
    let mut tx = 0.0;
    let mut ty = 0.0;

    let mut cx = 0;
    let mut cy = 0;

    for point in points {
        if point.is_empty() {
            continue;
        }
        cx += 1;
        cy += 1;

        let p = point.scale_transform(transform);
        tx += p.x();
        ty += p.y();
    }

    if cx == 0 {
        return None;
    }

    let mx = tx / cx as f64;
    let my = ty / cy as f64;

    Some((mx, my))
}

pub fn linear_regression(points: &[Point], transform: TransformType) -> Option<(f64, f64)> {
    let (mx, my) = calc_means(points, transform)?;

    let numerator = calc_numerator(points, transform, mx, my);
    let denom = calc_denom(points, transform, mx);

    let slope = numerator / denom;
    let intercept = my - slope * mx;

    Some((slope, intercept))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn input_empty() {
        let res = linear_regression(&[], TransformType::None);

        assert!(res.is_none());
    }

    #[test]
    fn input_simple() {
        let points = [
            Point(0.0, 10.0),
            Point(1.0, 15.0),
            Point(2.0, 20.0),
            Point(3.0, 25.0),
            Point(4.0, 30.0),
        ];

        let (slope, intercept) = linear_regression(&points, TransformType::None).unwrap();

        assert_eq!(slope, 5.0);
        assert_eq!(intercept, 10.0);
    }

    #[test]
    fn input_off_axis() {
        let points = [
            Point(1000.0, 1000.0),
            Point(1010.0, 1010.0),
            Point(1020.0, 1020.0),
            Point(1030.0, 1035.0),
            Point(1040.0, 1080.0),
        ];

        let (slope, intercept) = linear_regression(&points, TransformType::None).unwrap();

        assert_eq!(slope, 1.85);
        assert_eq!(intercept, -858.0);
    }

    #[test]
    fn input_xy() {
        let points = [
            Point(-3.0, -2.0),
            Point(-2.0, -1.0),
            Point(0.0, 1.0),
            Point(3.0, 3.0),
            Point(9.0, 9.0),
        ];

        let (slope, intercept) = linear_regression(&points, TransformType::None).unwrap();

        assert!((slope - 0.901).abs() < 0.001);
        assert!((intercept - 0.738197).abs() < 0.001);
    }

    #[test]
    fn input_log_x() {
        let points = [
            Point(0.0_f64.exp(), 0.0 + 50.0),
            Point(1.0_f64.exp(), 1.0 + 50.0),
            Point(2.0_f64.exp(), 2.0 + 50.0),
            Point(3.0_f64.exp(), 3.0 + 50.0),
            Point(4.0_f64.exp(), 4.0 + 50.0),
        ];

        let (slope, intercept) = linear_regression(&points, TransformType::LogX).unwrap();

        assert!((slope - 1.0).abs() < 0.001);
        assert!((intercept - 50.0).abs() < 0.001);
    }

    #[test]
    fn input_log_y_no_intercept() {
        let points = [
            Point(0.0, 0.0_f64.exp()),
            Point(1.0, 1.0_f64.exp()),
            Point(2.0, 2.0_f64.exp()),
            Point(3.0, 3.0_f64.exp()),
            Point(4.0, 4.0_f64.exp()),
        ];

        let (slope, intercept) = linear_regression(&points, TransformType::LogY).unwrap();

        assert!((slope - 1.0).abs() < 0.001);
        assert!((intercept - 0.0).abs() < 0.001);
    }

    #[test]
    fn input_log_y_intercept() {
        let points = [
            Point(0.0, 0.0_f64.exp() + 10.0),
            Point(1.0, 1.0_f64.exp() + 10.0),
            Point(2.0, 2.0_f64.exp() + 10.0),
            Point(3.0, 3.0_f64.exp() + 10.0),
            Point(4.0, 4.0_f64.exp() + 10.0),
        ];

        let (slope, intercept) = linear_regression(&points, TransformType::LogY).unwrap();

        assert!((slope - 0.440159).abs() < 0.001);
        assert!((intercept - 2.19348).abs() < 0.001);
    }
}
