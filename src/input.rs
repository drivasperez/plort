#![allow(unused)]
use crate::config::Config;
use crate::types::{DataSet, Point};
use std::io;
use std::io::prelude::*;
#[derive(Debug)]
pub enum ReadInputStatus {
    EndOfStream,
    DatasetComplete,
}

pub fn read_input(config: &Config, dataset: &mut DataSet) -> io::Result<ReadInputStatus> {
    let mut row_count = 0;
    let mut stdin = io::stdin().lock();

    for line in stdin.lines() {
        let line = line?;
        let line = line.trim();

        let res = process_line(config, dataset, line, row_count);
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
            ProcessLineResult::Done => {
                return Ok(ReadInputStatus::DatasetComplete);
            }
        }
    }

    Ok(ReadInputStatus::EndOfStream)
}
fn is_comment_marker(c: char) -> bool {
    c == '#' || c == '/'
}

fn number_head(c: char) -> bool {
    c.is_ascii_digit() || c == '-' || c == '+' || c == '.'
}

enum ProcessLineResult {
    Ok,
    Empty,
    Comment,
    Done,
}

fn process_line(
    config: &Config,
    dataset: &mut DataSet,
    line: &str,
    row_count: usize,
) -> ProcessLineResult {
    let mut col = 0;

    if line.is_empty() {
        return ProcessLineResult::Empty;
    }
    let chars = line.chars().collect::<Vec<char>>();

    let mut cur_x = row_count as f64;
    let mut has_x = false;

    let mut offset = 0;
    while offset < chars.len() {
        let mut v: Option<f64> = Some(0.0);
        let c = chars[offset];
        if is_comment_marker(c) {
            return ProcessLineResult::Comment;
        }

        if !number_head(c) {
            if offset == 0 {
                v = None;
            } else {
                offset += 1;
                if !number_head(c) {
                    v = None;
                }
            }
            return ProcessLineResult::Done;
        }

        let mut end = offset + 1;
        while end < chars.len() && number_head(chars[end]) {
            end += 1;
        }

        let num = &line[offset..end];
        let num = num.parse::<f64>().unwrap();
        v = Some(num);
        offset = end;

        if config.x_column && !has_x {
            cur_x = num;
            has_x = true;
        } else {
            if let Some(num) = v {
                let p = Point(cur_x as f64, num);
                dataset.add_pair(config, row_count, col, Some(p));
            } else {
                dataset.add_pair(config, row_count, col, None);
            }
            col += 1;
            if col == crate::MAX_COLUMNS {
                break;
            }
        }
    }

    // Fill remaining columns with None.
    while col < dataset.columns {
        dataset.add_pair(config, row_count, col, None);
        col += 1;
    }

    ProcessLineResult::Ok
}
