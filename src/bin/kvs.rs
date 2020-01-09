extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::exit;
use kvs::{Result, KvStore, KvError};

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

fn main() -> Result<()> {
    let matches = parse_args();
    if matches.is_present("version") {
        println!(env!("CARGO_PKG_VERSION"))
    };

    let mut kvs = KvStore::open(std::env::current_dir().unwrap())?;
    let (command, sub_matches) = matches.subcommand();
    let sub_matches = sub_matches.unwrap();
    let result = match command {
        "get" => {
            let key = sub_matches.value_of("key").unwrap().to_owned();
            match kvs.get(key) {
                Ok(value) => {
                    match value {
                        Some(v) => {
                            println!("{}", v);
                        },
                        None => println!("Key not found"),
                    }
                    Ok(())
                },
                Err(e) => Err(e)
            }
        }
        "set" => {
            kvs.set(sub_matches.value_of("key").unwrap().to_owned(),
                    sub_matches.value_of("value").unwrap().to_owned())
        }
        "rm" => {
            kvs.remove(sub_matches.value_of("key").unwrap().to_owned())
        }
        _ => {
            eprintln!("available commands, [get, set, rm]");
            exit(1);
        }
    };
    match result {
        Ok(()) => {},
        Err(KvError::KeyNotExists{key: _} ) => {
            println!("Key not found");
            exit(1);
        },
        Err(e) => panic!("Unexpected error occurs {:?}", e)
    };
    return Ok(())
}
