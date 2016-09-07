extern crate clap;
extern crate ews;
extern crate rusqlite;

use clap::{App, Arg, SubCommand};
use ews::db;
use rusqlite::Connection;

macro_rules! abort_on_error {
    ( $x:expr, $body:expr ) => {
        match $x {
            Ok(_) => $body,
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1);
            }
        }
    };
    ( $x:expr, $id:ident, $body:expr ) => {
        match $x {
            Ok($id) => $body,
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1);
            }
        }
    }
}

fn create_new_user(conn: &Connection) {
    println!("No users have been created yet. \
              Let's create one now.");
    abort_on_error!(ews::db::user::create_new_user(conn), {
        println!("User created.");
    });
}

macro_rules! with_current_user {
    ( $conn:expr, $user:ident, $body:expr ) => {
        match ews::db::user::current_user($conn) {
            None => { create_new_user($conn); },
            Some($user) => $body
        }
    }
}

fn main() {
    let matches = App::new("ews")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(SubCommand::with_name("all")
                    .about("Lists all open cases."))
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
        Some("all") => {
            let conn = ews::db::get_connection();
            with_current_user!(&conn, user, {
                abort_on_error!(
                    ews::db::case::all_open_cases(&conn, user.id), cases, {
                        println!("ID\tTITLE\tOPEN FOR");
                        for case in cases {
                            println!("{}\t{}\t{} days",
                                     case.id,
                                     case.title,
                                     ews::util::age_in_days(case.opened_date));
                        }
                });
            });
        },
        Some("open") => {
            let conn = ews::db::get_connection();
            with_current_user!(&conn, user, {
                let matches = matches.subcommand_matches("open").unwrap();
                let title = matches.value_of("title");
                abort_on_error!(
                    ews::db::case::create_new_case(&conn, title, user.id), {
                        println!("Case created.");
                    }
                );
            });
        },
        Some("setup") => {
            if !ews::config::ews_home_dir_exists() {
                let dir_name = ews::config::ews_home_dir().to_str().unwrap().to_owned();
                println!("Creating {}...", dir_name);
                abort_on_error!(ews::config::create_ews_home_dir(), {});
            }

            println!("Setting up ews db...");
            abort_on_error!(db::run_migrations(), {});

            println!("Setup was successful.");
        },
        Some("user") => {
            let conn = ews::db::get_connection();
            with_current_user!(&conn, user, {
                println!("Current user: {}", user.name);
            });
        },
        Some(_) => {
            panic!("this should never happen");
        }
        None => {
            println!("TODO: same as `ews info`")
        }
    }
}
