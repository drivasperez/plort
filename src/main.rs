use anyhow::Context;
use clap::Parser;
use config::Config;
use draw::draw;
use input::{read_input, ReadInputStatus};
use types::DataSet;

mod ascii;
mod config;
mod draw;
mod input;
mod regression;
mod scale;
mod svg;
mod types;

const MAX_COLUMNS: u8 = 255;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let config = Config::parse();
    let mut dataset = DataSet::default();

    let mut end_of_stream = false;
    while !end_of_stream {
        let status = read_input(&config, &mut dataset).context("Reading input")?;
        if let ReadInputStatus::EndOfStream = status {
            end_of_stream = true;
        }
        if dataset.rows == 0 {
            // No input
            return Ok(());
        }

        draw(&config, &dataset).context("Drawing diagram")?;
        if !end_of_stream {
            println!();
        }
    }

    Ok(())
}
