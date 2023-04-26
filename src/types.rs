use crate::config::Config;

#[derive(Debug)]
pub struct Point(pub f64, pub f64);

#[derive(Debug)]
pub struct DataSet {
    pub row_ceil: u8,
    pub columns: u8,
    pub rows: usize,
    pub points: Vec<Vec<Option<Point>>>, // p[column][row]
}

impl Default for DataSet {
    fn default() -> Self {
        DataSet {
            row_ceil: 1,
            columns: 1,
            rows: 0,
            points: Vec::new(),
        }
    }
}

impl DataSet {
    pub fn add_pair(&mut self, config: &Config, row: usize, col: u8, point: Option<Point>) {
        // Add columns, padding with None as necessary
        while col >= self.columns {
            let mut v = Vec::with_capacity(self.rows);
            for _ in 0..self.rows {
                v.push(None);
            }

            self.points.push(v);
            self.columns += 1;
        }

        // Add rows, padding with None as necessary
        while row >= self.rows {
            for v in &mut self.points {
                v.push(None);
            }
            self.rows += 1;
        }

        if config.flip_xy {
            if let Some(point) = point {
                self.points[col as usize][row] = Some(Point(point.1, point.0));
            } else {
                self.points[col as usize][row] = None;
            }
        } else {
            self.points[col as usize][row] = point;
        }
    }
}
