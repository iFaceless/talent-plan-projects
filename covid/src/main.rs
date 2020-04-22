use std::path::PathBuf;
use structopt::StructOpt;

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

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}

