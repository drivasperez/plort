use crate::config::Config;
use crate::types::{DataSet, Point, EMPTY_VALUE};
use anyhow::Context;
use std::io::prelude::*;
#[derive(Debug)]
pub enum ReadInputStatus {
    EndOfStream,
    DatasetComplete,
}

pub fn read_input(
    config: &Config,
    dataset: &mut DataSet,
    reader: &mut impl BufRead,
) -> anyhow::Result<ReadInputStatus> {
    let mut row_count = 0;

    for line in reader.lines() {
        let line = line.context("Read line from stdin")?;

        let res = process_line(config, dataset, &line, row_count);
        match res {
            ProcessLineResult::Ok => {
                row_count += 1;
            }
            ProcessLineResult::Empty => {
                if config.stream_mode {
                    return Ok(ReadInputStatus::DatasetComplete);
                } else {
                    return Ok(ReadInputStatus::EndOfStream);
                }
            }
            ProcessLineResult::Comment => continue,
        }
    }

    Ok(ReadInputStatus::EndOfStream)
}
fn is_comment_marker(c: char) -> bool {
    c == '#' || c == '/'
}

fn number_head(c: char) -> bool {
    c.is_ascii_digit() || c == '-' || c == '+' || c == '.' || c == 'e' || c == 'E'
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProcessLineResult {
    Ok,
    Empty,
    Comment,
}

pub fn process_line(
    config: &Config,
    dataset: &mut DataSet,
    line: &str,
    row_count: usize,
) -> ProcessLineResult {
    let line = line.trim_end();

    if line.is_empty() {
        return ProcessLineResult::Empty;
    }

    let mut col = 0;
    let content = match line.split_once(is_comment_marker) {
        Some((data, _comment)) => data,
        None => line,
    };

    if content.is_empty() {
        return ProcessLineResult::Comment;
    }

    let mut cur_x = row_count as f64;
    let mut has_x = false;

    let nums = content.split(|c| !number_head(c)).collect::<Vec<_>>();
    if nums.len() == 2 && nums.iter().all(|x| x.is_empty()) {
        // Slightly annoying special case, where if the string is a single invalid character,
        // split will return two empty strings, and we want to treat this as a single
        // empty value.
        let point = Point(cur_x, EMPTY_VALUE);
        dataset.add_pair(config, row_count, col, point);
        col += 1;
    } else {
        for num in nums {
            let v = num.parse::<f64>().unwrap_or(EMPTY_VALUE);

            if config.x_column && !has_x {
                cur_x = v;
                has_x = true;
            } else {
                let point = Point(cur_x, v);
                dataset.add_pair(config, row_count, col, point);
                col += 1;
            }
        }
    }

    // Fill remaining columns with EMPTY_VALUE.
    while col < dataset.columns {
        dataset.add_pair(config, row_count, col, Point(cur_x, EMPTY_VALUE));
        col += 1;
    }

    ProcessLineResult::Ok
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_empty() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let line = "\n";
        let res = process_line(&config, &mut dataset, "", 0);
        assert_eq!(res, ProcessLineResult::Empty);
        let res = process_line(&config, &mut dataset, line, 0);
        assert_eq!(res, ProcessLineResult::Empty);
    }

    #[test]
    fn input_comment() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let c1 = "# comment\n";
        let c2 = "// comment\n";
        let res = process_line(&config, &mut dataset, c1, 0);
        assert_eq!(res, ProcessLineResult::Comment);
        let res = process_line(&config, &mut dataset, c2, 0);
        assert_eq!(res, ProcessLineResult::Comment);
    }

    #[test]
    fn input_null() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let line = "_";
        let res = process_line(&config, &mut dataset, line, 0);
        assert_eq!(res, ProcessLineResult::Ok);
        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 1);
        assert_eq!(dataset.points[0][0], Point(0.0, EMPTY_VALUE));
    }

    #[test]
    fn input_single() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let line = "1\n";
        let res = process_line(&config, &mut dataset, line, 0);
        assert_eq!(res, ProcessLineResult::Ok);
        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 1);
        assert_eq!(dataset.points[0][0], Point(0.0, 1.0));
    }

    #[test]
    fn input_pair() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let line = "1 2\n";
        let res = process_line(&config, &mut dataset, line, 0);
        assert_eq!(res, ProcessLineResult::Ok);
        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 2);
        assert_eq!(dataset.points[0][0], Point(0.0, 1.0));
        assert_eq!(dataset.points[1][0], Point(0.0, 2.0));
    }

    #[test]
    fn input_floats() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let line = "99.9 999.999\n";
        let res = process_line(&config, &mut dataset, line, 0);
        assert_eq!(res, ProcessLineResult::Ok);
        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 2);
        assert_eq!(dataset.points[0][0], (Point(0.0, 99.9)));
        assert_eq!(dataset.points[1][0], (Point(0.0, 999.999)));
    }

    #[test]
    fn input_csv() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let line = "1,2\n";
        let res = process_line(&config, &mut dataset, line, 0);
        assert_eq!(res, ProcessLineResult::Ok);
        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 2);
        assert_eq!(dataset.points[0][0], (Point(0.0, 1.0)));
        assert_eq!(dataset.points[1][0], (Point(0.0, 2.0)));
    }

    #[test]
    fn input_tab() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let line = "1\t2\n";
        let res = process_line(&config, &mut dataset, line, 0);
        assert_eq!(res, ProcessLineResult::Ok);
        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 2);
        assert_eq!(dataset.points[0][0], (Point(0.0, 1.0)));
        assert_eq!(dataset.points[1][0], (Point(0.0, 2.0)));
    }

    #[test]
    fn input_exponent() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        let line = "-24e-7,+50e5";
        let res = process_line(&config, &mut dataset, line, 0);
        assert_eq!(res, ProcessLineResult::Ok);
        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 2);
        assert_eq!(dataset.points[0][0], (Point(0.0, -24e-7)));
        assert_eq!(dataset.points[1][0], (Point(0.0, 50e5)));
    }

    #[test]
    fn input_multiline() {
        let config = Config::default();
        let mut dataset = DataSet::default();

        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, "23", 0)
        );
        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, "24", 1)
        );

        assert_eq!(dataset.rows, 2);
        assert_eq!(dataset.columns, 1);
        assert_eq!(dataset.points[0][0], (Point(0.0, 23.0)));
        assert_eq!(dataset.points[0][1], (Point(1.0, 24.0)));
    }

    #[test]
    fn input_single_column_null() {
        let config = Config::default();
        let mut dataset = DataSet::default();

        let lines = vec!["1278", "377", "316", "232", "_", "93", "63", "11"];
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(
                ProcessLineResult::Ok,
                process_line(&config, &mut dataset, line, i)
            );
        }

        assert_eq!(dataset.rows, 8);
        assert_eq!(dataset.columns, 1);
        assert_eq!(dataset.points[0][0], (Point(0.0, 1278.0)));
        assert_eq!(dataset.points[0][1], (Point(1.0, 377.0)));
        assert_eq!(dataset.points[0][2], (Point(2.0, 316.0)));
        assert_eq!(dataset.points[0][3], (Point(3.0, 232.0)));
        assert_eq!(dataset.points[0][4], (Point(4.0, EMPTY_VALUE)));
        assert_eq!(dataset.points[0][5], (Point(5.0, 93.0)));
        assert_eq!(dataset.points[0][6], (Point(6.0, 63.0)));
        assert_eq!(dataset.points[0][7], (Point(7.0, 11.0)));
    }

    #[test]
    fn input_multiline_null() {
        let config = Config::default();
        let mut dataset = DataSet::default();

        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, "1,2,3", 0)
        );

        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, "4,,6", 1)
        );

        assert_eq!(dataset.rows, 2);
        assert_eq!(dataset.columns, 3);

        let exp0_0 = Point(0.0, 1.0);
        let exp0_1 = Point(0.0, 2.0);
        let exp0_2 = Point(0.0, 3.0);

        let exp1_0 = Point(1.0, 4.0);
        let exp1_1 = Point(1.0, EMPTY_VALUE);
        let exp1_2 = Point(1.0, 6.0);

        assert_eq!(dataset.points[0][0], exp0_0);
        assert_eq!(dataset.points[1][0], exp0_1);
        assert_eq!(dataset.points[2][0], exp0_2);

        assert_eq!(dataset.points[0][1], exp1_0);
        assert_eq!(dataset.points[1][1], exp1_1);
        assert_eq!(dataset.points[2][1], exp1_2);
    }

    #[test]
    fn input_trailing_null() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, "1,2,3,", 0)
        );

        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 4);

        let exp0_0 = Point(0.0, 1.0);
        let exp0_1 = Point(0.0, 2.0);
        let exp0_2 = Point(0.0, 3.0);
        let exp0_3 = Point(0.0, EMPTY_VALUE);

        assert_eq!(dataset.points[0][0], exp0_0);
        assert_eq!(dataset.points[1][0], exp0_1);
        assert_eq!(dataset.points[2][0], exp0_2);
        assert_eq!(dataset.points[3][0], exp0_3);
    }

    #[test]
    fn input_leading_null() {
        let config = Config::default();
        let mut dataset = DataSet::default();
        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, ",1,2,3", 0)
        );

        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 4);

        let exp0_0 = Point(0.0, EMPTY_VALUE);
        let exp0_1 = Point(0.0, 1.0);
        let exp0_2 = Point(0.0, 2.0);
        let exp0_3 = Point(0.0, 3.0);

        assert_eq!(dataset.points[0][0], exp0_0);
        assert_eq!(dataset.points[1][0], exp0_1);
        assert_eq!(dataset.points[2][0], exp0_2);
        assert_eq!(dataset.points[3][0], exp0_3);
    }

    #[test]
    fn row_with_more_columns_pads_previous_with_nulls() {
        let config = Config::default();
        let mut dataset = DataSet::default();

        // Pascal's triangle
        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, "1", 0)
        );
        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, "1,1", 1)
        );
        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, "1,2,1", 2)
        );

        // Expected data, exp[col][row]
        let points: [[Point; 3]; 3] = [
            [(Point(0.0, 1.0)), (Point(1.0, 1.0)), (Point(2.0, 1.0))],
            [Point(0.0, EMPTY_VALUE), Point(1.0, 1.0), Point(2.0, 2.0)],
            [
                Point(0.0, EMPTY_VALUE),
                Point(1.0, EMPTY_VALUE),
                Point(2.0, 1.0),
            ],
        ];

        for (i, col) in points.iter().enumerate() {
            for (j, point) in col.iter().enumerate() {
                assert_eq!(dataset.points[i][j], *point);
            }
        }
    }

    #[test]
    fn x_column_negative_numbers() {
        let config = Config {
            x_column: true,
            ..Default::default()
        };

        let mut dataset = DataSet::default();

        let line = "-300 -400";

        assert_eq!(
            ProcessLineResult::Ok,
            process_line(&config, &mut dataset, line, 0)
        );

        assert_eq!(dataset.rows, 1);
        assert_eq!(dataset.columns, 1);

        let exp0_0 = Point(-300.0, -400.0);

        assert_eq!(dataset.points[0][0], exp0_0);
    }
}
