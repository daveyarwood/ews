mod migrations;

use config;
use rusqlite::Connection;
use schemamama_rusqlite::SqliteMigrationError;

pub fn get_connection() -> Connection {
    Connection::open(config::ews_db_file()).unwrap()
}

pub fn run_migrations() -> Result<(), SqliteMigrationError> {
    let conn = get_connection();
    migrations::run(&conn)
}
