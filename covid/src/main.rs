//! A handful command line tool to query covid-19 infections around the world.
use csv;
use serde::Deserialize;
use std::{error, fs, io, path::PathBuf, process};
use structopt::StructOpt;

/// Opt collects the command line arguments
#[derive(Debug, StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
#[structopt(version = env!("CARGO_PKG_VERSION"))]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
#[structopt(author = env!("CARGO_PKG_AUTHORS"))]
struct Opt {
    /// Query data of which country
    #[structopt(value_name = "COUNTRY")]
    country: String,

    /// Input data file
    #[structopt(long, short, parse(from_os_str), value_name = "DATA_PATH")]
    data_path: Option<PathBuf>,

    /// Don't show noisy messages
    #[structopt(long, short)]
    quiet: bool,
}

/// Record represents a row in the target csv file
#[derive(Debug, Deserialize)]
struct Record {
    country: String,
    number_of_newly_diagnosis: u32,
    number_of_cumulative_diagnosis: u32,
    number_of_current_diagnosis: u32,
    number_of_deaths: u32,
    number_of_cures: u32,
}

fn main() {
    run();
}

fn run() {
    let opt = Opt::from_args();
    let input: Box<dyn io::Read> = match opt.data_path {
        None => Box::new(io::stdin()),
        Some(p) => Box::new(fs::File::open(p.as_path()).unwrap()),
    };
    let mut rdr = csv::Reader::from_reader(input);
    dbg!(rdr.headers().unwrap());

    for r in rdr.deserialize() {
        let record: Record = r.unwrap();
        if record.country == opt.country {
            dbg!(&record);
            return
        }
    }

    println!("country '{}' not found", opt.country);
}