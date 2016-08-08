extern crate clap;
extern crate ews;

use clap::{App, SubCommand};
use ews::db;

// https://github.com/kbknapp/clap-rs/blob/master/examples/08_subcommands.rs

fn main() {
    let matches = App::new("ews")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(SubCommand::with_name("setup")
                    .about("Bootstraps the ews database file."))
        .subcommand(SubCommand::with_name("user")
                    .about("Displays information about the current user."))
        .get_matches();

    match matches.subcommand_name() {
        Some("setup") => {
            print!("Setting up ews db... ");
            db::run_migrations().unwrap();
            println!("done.");
        },
        Some("user") => {
            println!("TODO: user");
        },
        Some(_) => {
            panic!("this should never happen");
        }
        None => {
            println!("TODO: same as `ews info`")
        }
    }
}
