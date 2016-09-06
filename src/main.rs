extern crate clap;
extern crate ews;
extern crate rusqlite;

use clap::{App, Arg, SubCommand};
use ews::db;
use rusqlite::Connection;

// https://github.com/kbknapp/clap-rs/blob/master/examples/08_subcommands.rs

fn create_new_user(conn: &Connection) {
    println!("No users have been created yet. \
              Let's create one now.");
    match ews::db::user::create_new_user(conn) {
        Ok(()) => { println!("User created."); },
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let matches = App::new("ews")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(SubCommand::with_name("open")
                    .about("Opens a new case.")
                    .arg(Arg::with_name("title")
                         .help("the case title")
                         .index(1)))
        .subcommand(SubCommand::with_name("setup")
                    .about("Bootstraps the ews database file."))
        .subcommand(SubCommand::with_name("user")
                    .about("Displays information about the current user."))
        .get_matches();

    if !(ews::config::ews_home_dir_exists()
         && ews::config::db_file_exists()) &&
        matches.subcommand_name() != Some("setup") {
        println!("ews home directory and/or db file not found.");
        println!("Please run `ews setup` to get things set up.");
        std::process::exit(1);
    }

    match matches.subcommand_name() {
        Some("open") => {
            let conn = ews::db::get_connection();
            match ews::db::user::current_user(&conn) {
                None => { create_new_user(&conn); },
                Some(user) => {
                    let matches = matches.subcommand_matches("open").unwrap();
                    match ews::db::case::create_new_case(&conn,
                                                         matches.value_of("title"),
                                                         user.id) {
                        Err(e) => {
                            println!("{:?}", e);
                            std::process::exit(1);
                        },
                        Ok(_) => {
                            println!("Case created.");
                        }
                    }
                }
            }
        },
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
                println!("ERROR: Failed to run db migrations.");
                std::process::exit(1);
            }

            println!("Setup was successful.");
        },
        Some("user") => {
            let conn = ews::db::get_connection();
            match ews::db::user::current_user(&conn) {
                None => { create_new_user(&conn); },
                Some(user) => { println!("Current user: {}", user.name); }
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
