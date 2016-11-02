extern crate clap;
extern crate ews;
extern crate rusqlite;

use clap::{App, Arg, SubCommand};
use ews::{config, db, util};
use ews::query::Query;
use rusqlite::Connection;

macro_rules! abort_on_error {
    ( $x:expr ) => {
        match $x {
            Ok(_) => {},
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1);
            }
        }
    };
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
    abort_on_error!(db::user::create_new_user(conn), {
        println!("User created.");
    });
}

macro_rules! with_current_user {
    ( $conn:expr, $user:ident, $body:expr ) => {
        match db::user::current_user($conn) {
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
        .subcommand(SubCommand::with_name("close")
                    .about("Closes an open case.")
                    .arg(Arg::with_name("case")
                         .help("a case ID or search string")
                         .index(1)))
        .subcommand(SubCommand::with_name("info")
                    .about("Displays a summary of the status of your cases.")
                    .arg(Arg::with_name("case")
                         .help("a case ID or search string")
                         .index(1)))
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

    if !(config::ews_home_dir_exists()
         && config::db_file_exists()) &&
        matches.subcommand_name() != Some("setup") {
        println!("ews home directory and/or db file not found.");
        println!("Please run `ews setup` to get things set up.");
        std::process::exit(1);
    }

    match matches.subcommand_name() {
        Some("all") => {
            let conn = db::get_connection();
            with_current_user!(&conn, user, {
                abort_on_error!(
                    db::case::all_open_cases(&conn, user.id), cases, {
                        db::case::print_cases(cases);
                });
            });
        },
        Some("close") => {
            let matches = matches.subcommand_matches("close").unwrap();
            let case_query = match matches.value_of("case") {
                Some(query) => Query::new(query.to_string()),
                None => {
                    let query = util::prompt(
                        "Please enter a case ID or part of the title: ");
                    Query::new(query)
                }
            };

            let query_type = match case_query {
                Query::Id(_) => "ID",
                Query::SearchString(_) => "search string"
            };

            let conn = db::get_connection();
            with_current_user!(&conn, user, {
                abort_on_error!(
                    db::case::find_case(&conn, user.id, case_query, true),
                                        result, {
                    match result {
                        Some(case) => {
                            abort_on_error!(
                                db::case::close_case(&conn, case.id), {
                                    println!("\nCase closed.");
                            });
                        },
                        None => {
                            println!("No open case found with that {}.", query_type);
                            std::process::exit(1);
                        }
                    }
                });
            });
        },
        Some("info") => {
            let conn = db::get_connection();
            with_current_user!(&conn, user, {
                let matches = matches.subcommand_matches("info").unwrap();
                match matches.value_of("case") {
                    None => {
                        abort_on_error!(
                            db::summary::print_summary(&conn, user.id)
                        );
                    },
                    Some(query) => {
                        let case_query = Query::new(query.to_string());
                        abort_on_error!(
                            db::case::print_summary(&conn, user.id, case_query)
                        );
                    }
                }
            });
        },
        Some("open") => {
            let matches = matches.subcommand_matches("open").unwrap();
            let title = matches.value_of("title");

            let conn = db::get_connection();
            with_current_user!(&conn, user, {
                abort_on_error!(
                    db::case::open_case(&conn, title, user.id), {
                        println!("Case created.");
                    }
                );
            });
        },
        Some("setup") => {
            if !config::ews_home_dir_exists() {
                let dir_name = config::ews_home_dir().to_str().unwrap().to_owned();
                println!("Creating {}...", dir_name);
                abort_on_error!(config::create_ews_home_dir(), {});
            }

            println!("Setting up ews db...");
            abort_on_error!(db::run_migrations(), {});

            println!("Setup was successful.");
        },
        Some("user") => {
            let conn = db::get_connection();
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
