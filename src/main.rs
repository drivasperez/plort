use clap::Parser;
use config::Config;
use input::{read_input, ReadInputStatus};
use std::io;
use types::DataSet;

mod config;
mod input;
mod types;

const MAX_COLUMNS: u8 = 255;

fn main() -> io::Result<()> {
    let config = Config::parse();
    println!("{:?}", config);
    let mut dataset = DataSet::default();

    let mut end_of_stream = false;
    while !end_of_stream {
        let status = read_input(&config, &mut dataset)?;
        if let ReadInputStatus::EndOfStream = status {
            end_of_stream = true;
        }
        if dataset.rows == 0 {
            // No input
            return Ok(());
        }

        draw(&config, &dataset)?;
        if !end_of_stream {
            println!();
        }
    }

    Ok(())
}

fn draw(config: &Config, dataset: &DataSet) -> io::Result<()> {
    todo!()
}
