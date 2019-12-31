extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::exit;

fn parse_args() -> ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("version")
                .short("V")
                .long("version")
                .takes_value(false),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get value from key value store")
                .arg(
                    Arg::with_name("key")
                        .help("search key")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("key").required(true).index(1))
                .arg(Arg::with_name("value").required(true).index(2)),
        )
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("key").required(true).index(1)))
        .get_matches()
}

fn main() {
    let matches = parse_args();
    if matches.is_present("version") {
        println!(env!("CARGO_PKG_VERSION"))
    } else if let Some(subcommand_name) = matches.subcommand_name() {
        match subcommand_name {
            "get" => {
                eprintln!("unimplemented");
                exit(1);
            }
            "set" => {
                eprintln!("unimplemented");
                exit(1);
            }
            "rm" => {
                eprintln!("unimplemented");
                exit(1);
            }
            _ => {
                eprintln!("available commands, [get, set, rm]");
                exit(1);
            }
        }
    } else {
        eprintln!("command required, [get, set, rm]");
        exit(1);
    }

    println!("{:?}", matches.is_present("version"));
}
