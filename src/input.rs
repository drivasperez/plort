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
        let mut line = line?;

        // let res = process_line(config, &dataset, &mut line, row_count);
    }

    todo!()
}
fn is_comment_marker(c: char) -> bool {
    c == '#' || c == '/'
}

fn number_head(c: char) -> bool {
    c.is_ascii_digit() || c == '-' || c == '+' || c == '.'
}
