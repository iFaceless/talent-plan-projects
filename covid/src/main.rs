//! A handful command line tool to query covid-19 infections around the world.
use csv;
use serde::Deserialize;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process,
};
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
    let opt = Opt::from_args();
    match search(&opt.data_path.map(|x| x.as_path().to_owned()), &opt.country) {
        Ok(r) => println!("{:?}", r),
        Err(e) => {
            println!("{:?}", e);
            process::exit(1);
        },
    }
}

fn search<P: AsRef<Path>>(
    input: &Option<P>,
    country: &str,
) -> Result<Record, Box<dyn std::error::Error>> {
    let input: Box<dyn io::Read> = match input {
        None => Box::new(io::stdin()),
        Some(p) => Box::new(fs::File::open(p)?),
    };
    let mut rdr = csv::Reader::from_reader(input);
    for r in rdr.deserialize() {
        let record: Record = r?;
        if record.country == country {
            return Ok(record);
        }
    }

    Err(From::from("No matching country found."))
}
