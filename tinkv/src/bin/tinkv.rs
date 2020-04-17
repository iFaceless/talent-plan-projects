use clap::{App, Arg, SubCommand, AppSettings};
use std::process::exit;

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("get")
                .about("Get value from store")
                .arg(
                    Arg::with_name("key")
                        .required(true)
                        .help("A string key")
                        .value_name("KEY"),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set key value to store")
                .arg(
                    Arg::with_name("key")
                        .help("A string key")
                        .required(true)
                        .value_name("KEY"),
                )
                .arg(
                    Arg::with_name("value")
                        .help("A string value")
                        .required(true)
                        .value_name("VALUE"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove key value from store")
                .arg(
                    Arg::with_name("key")
                        .help("A string key")
                        .required(true)
                        .value_name("KEY"),
                ),
        );

    let m = app.get_matches();
    match m.subcommand() {
        ("get", Some(_)) => unimplemented(),
        ("set", Some(_)) => unimplemented(),
        ("rm", Some(_)) => unimplemented(),
        _ => unreachable!(),
    }
}

fn unimplemented() {
    eprintln!("unimplemented");
    exit(1);
}
