use clap::{App, AppSettings, Arg, SubCommand};
use std::process::exit;
use structopt::StructOpt;

/// Subcommands
#[derive(StructOpt, Debug)]
enum Command {
    /// Get value from store
    Get {
        /// A string key
        key: String,
    },
    /// Save key value to store
    Set {
        /// A string key
        key: String,
        /// A string value
        value: String,
    },
    #[structopt(name = "rm")]
    /// Remove value from store
    Remove {
        /// A string key
        key: String,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(
    rename_all = "kebab-case",
    name = env!("CARGO_PKG_NAME"), 
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

fn main() {
    let opt = Opt::from_args();
    match opt.command {
        Command::Get { key: _key } => unimplemented(),
        Command::Set { key: _key, value: _value } => unimplemented(),
        Command::Remove { key: _value} => unimplemented(),
    }
}

fn unimplemented() {
    eprintln!("unimplemented");
    exit(1);
}
