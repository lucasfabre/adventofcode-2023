use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(clap::ValueEnum, Clone)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    pub part: Part,
    #[arg(short, long)]
    pub input_file: Option<String>,
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn get_input_stream(cli: &Cli) -> Box<dyn BufRead> {
    match &cli.input_file {
        Some(file_name) => {
            let f = File::open(file_name).expect("Could not open input file");
            Box::new(BufReader::new(f))
        }
        None => Box::new(BufReader::new(std::io::stdin())),
    }
}

pub fn init_logger(cli :&Cli) {
    let log_level = match cli.verbose {
        true => log::LevelFilter::max(),
        false => log::LevelFilter::Info,
    };

    let _ = env_logger::builder().filter_level(log_level).init();
}

pub fn init_tests() {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
}
