use crate::config::Config;

pub const EMPTY_VALUE: f64 = std::f64::NAN;

#[derive(Debug)]
pub struct Point(pub f64, pub f64);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
            || self.0.is_nan() && other.0.is_nan()
            || self.1.is_nan() && other.1.is_nan()
    }
}

#[derive(Debug)]
pub struct DataSet {
    pub row_ceil: u8,
    pub columns: u8,
    pub rows: usize,
    pub points: Vec<Vec<Point>>, // p[column][row]
}

impl Default for DataSet {
    fn default() -> Self {
        DataSet {
            row_ceil: 1,
            columns: 0,
            rows: 0,
            points: Vec::new(),
        }
    }
}

impl DataSet {
    pub fn add_pair(&mut self, config: &Config, row: usize, col: u8, point: Point) {
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
