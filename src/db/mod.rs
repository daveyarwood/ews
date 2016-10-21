mod migrations;
pub mod case;
pub mod item;
pub mod summary;
pub mod user;

use config;
use rusqlite::Connection;

pub fn get_connection() -> Connection {
    Connection::open(config::db_file()).unwrap()
}

pub fn run_migrations() -> Result<(), migrations::MigrationError> {
    let conn = get_connection();
    migrations::run(&conn)
}
