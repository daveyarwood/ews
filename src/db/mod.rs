mod migrations;

use config;
use rusqlite::Connection;

pub fn get_connection() -> Connection {
    Connection::open(config::ews_db_file()).unwrap()
}

pub fn run_migrations() -> Result<(), migrations::MigrationError> {
    config::create_ews_home_dir();
    let conn = get_connection();
    migrations::run(&conn)
}
