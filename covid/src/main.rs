//! A handful command line tool to query covid-19 infections around the world.
use csv;
use serde::Deserialize;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process,
};
use structopt::StructOpt;
use thiserror::Error;
use anyhow;

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

#[derive(Error, Debug)]
enum CliError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error("no matching record found")]
    NotFound,
}

fn main() {
    const FILEPATH: &str = "/Users/chris/Projects/Rust/open_source/pingcap-tp/covid/assets/covid-19-infections-20200422.csv";
    match search(FILEPATH, "美国") {
        Ok(r) => {
            println!("{:?}", r);
        },
        Err(e) => {
            eprintln!("{:?}", e);
        }
    };
}

fn search<P: AsRef<Path>>(filepath: P, country: &str) -> Result<Record, CliError> {
    let input= fs::File::open(filepath)?;
    let mut rdr = csv::Reader::from_reader(input);
    for r in rdr.deserialize() {
        let record: Record = r?;
        if record.country == country {
            return Ok(record);
        }
    }

    Err(CliError::NotFound)
}