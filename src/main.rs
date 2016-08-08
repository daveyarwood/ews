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
            if !ews::config::ews_home_dir_exists() {
                let dir_name = ews::config::ews_home_dir().to_str().unwrap().to_owned();
                println!("Creating {}...", dir_name);
                if ews::config::create_ews_home_dir().is_err() {
                    println!("ERROR: Unable to create directory: {}", dir_name);
                    std::process::exit(1);
                }
            }

            println!("Setting up ews db...");
            if db::run_migrations().is_err() {
                println!("\nERROR: Failed to run db migrations.");
                std::process::exit(1);
            }

            println!("Setup was successful.");
        },
        Some("user") => {
            let conn = ews::db::get_connection();
            match ews::db::user::current_user(&conn) {
                None => {
                    println!("No users have been created yet. \
                              Let's create one now.");
                    match ews::db::user::create_new_user(&conn) {
                        Ok(()) => { println!("User created."); },
                        Err(_) => { println!("ERROR: Unable to create user."); }
                    }
                },
                Some(user) => {
                    println!("Current user: {}", user.name);
                }
            }
        },
        Some(_) => {
            panic!("this should never happen");
        }
        None => {
            println!("TODO: same as `ews info`")
        }
    }
}
