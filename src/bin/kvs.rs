extern crate clap;

use clap::{App, Arg};
use std::process;
use kvs::KvStore;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            App::new("set")
                .about("associates value with a key")
                .arg(Arg::with_name("KEY")
                     .required(true)
                     .index(1))
                .arg(Arg::with_name("VALUE")
                     .required(true)
                     .index(2)))
        .subcommand(
            App::new("get")
                .about("returns the value associated to the key")
                .arg(Arg::with_name("KEY")
                     .required(true)
                     .index(1)))
        .subcommand(
            App::new("rm")
                .about("removes the value associated to the key")
                .arg(Arg::with_name("KEY")
                     .required(true)
                     .index(1)))
 
        .get_matches();

    let mut store = KvStore::new();
    
    if let Some(matches) = matches.subcommand_matches("set") {
        if matches.is_present("KEY") && matches.is_present("VALUE") {
            store.set(
                matches.value_of("KEY").unwrap().to_string(), 
                matches.value_of("VALUE").unwrap().to_string());
            println!("Done");
            process::exit(0x0);
        } else {
            eprintln!("Invalid arguments");
            process::exit(0x0111);
        }
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        if matches.is_present("KEY") {
            let value = store.get(matches.value_of("KEY").unwrap().to_string());
            println!("--> {:?}", value);
            process::exit(0x0);
        } else {
            eprintln!("Invalid arguments");
            process::exit(0x0111);
        }
    }

    if let Some(matches) = matches.subcommand_matches("rm") {
        if matches.is_present("KEY") {
            store.remove(matches.value_of("KEY").unwrap().to_string());
            println!("Done");
            process::exit(0x0);
        } else {
            println!("Invalid arguments");
            process::exit(0x0111);
        }
    }

    unimplemented!();
}
