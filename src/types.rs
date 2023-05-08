use crate::config::Config;
use crate::scale::TransformType;

pub const EMPTY_VALUE: f64 = std::f64::NAN;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
            || self.0.is_nan() && other.0.is_nan()
            || self.1.is_nan() && other.1.is_nan()
    }
}

impl Point {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_nan() || self.1.is_nan()
    }

    /// Scale the point according to the given log transform.
    pub fn scale_transform(&self, transform: TransformType) -> Point {
        let (log_x, log_y) = match transform {
            TransformType::None => (false, false),
            TransformType::LogX => (true, false),
            TransformType::LogY => (false, true),
            TransformType::LogXY => (true, true),
        };

        let x = if log_x && self.0 != 0.0 {
            self.0.ln()
        } else {
            self.0
        };

        let y = if log_y && self.1 != 0.0 {
            self.1.ln()
        } else {
            self.1
        };

        Point(x, y)
    }
}

#[derive(Debug)]
pub struct DataSet {
    pub columns: usize,
    pub rows: usize,
    pub points: Vec<Vec<Point>>, // p[column][row]
}

impl Default for DataSet {
    fn default() -> Self {
        DataSet {
            columns: 0,
            rows: 0,
            points: Vec::new(),
        }
    }
}

impl DataSet {
    pub fn add_pair(&mut self, config: &Config, row: usize, col: usize, point: Point) {
        // Add columns, padding with None as necessary
        while col >= self.columns {
            let mut v = Vec::with_capacity(self.rows);
            for _ in 0..self.rows {
                v.push(Point(EMPTY_VALUE, EMPTY_VALUE));
            }

            self.points.push(v);
            self.columns += 1;
        }

        // Add rows, padding with None as necessary
        while row >= self.rows {
            for col in 0..self.columns {
                self.points[col as usize].push(Point(EMPTY_VALUE, EMPTY_VALUE));
            }
            self.rows += 1;
        }

        if config.flip_xy {
            self.points[col as usize][row] = Point(point.1, point.0);
        } else {
            self.points[col as usize][row] = point;
        }
    }
}
