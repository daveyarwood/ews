use rusqlite;
use schemamama;
use schemamama_rusqlite::{SqliteAdapter, SqliteMigration, SqliteMigrationError};

pub type MigrationError = schemamama::Error<SqliteMigrationError>;

macro_rules! ews_migration {
    ( $version:expr, $migration:ident, $desc:expr, $up:expr, $down:expr) => {
        struct $migration;
        migration!($migration, $version, $desc);

        impl SqliteMigration for $migration {
            fn up(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute($up, &[]).map(|_| ())
            }

            fn down(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute($down, &[]).map(|_| ())
            }
        }
    }
}

ews_migration!(1470499978, CreateUsers, "create users table",
               "CREATE TABLE ews_user(\
                  id INTEGER PRIMARY KEY, \
                  name TEXT NOT NULL\
                );",
                "DROP TABLE ews_user;");

ews_migration!(1470499979, CreateCases, "create cases table",
               "CREATE TABLE ews_case(\
                  id INTEGER PRIMARY KEY, \
                  title TEXT NOT NULL, \
                  userid INTEGER NOT NULL, \
                  openeddate INTEGER NOT NULL, \
                  closeddate INTEGER, \
                  FOREIGN KEY(userid) REFERENCES ews_user(id)\
                );",
                "DROP TABLE ews_case;");

ews_migration!(1470499980, CreateItems, "create items table",
               "CREATE TABLE ews_item(\
                  id INTEGER PRIMARY KEY, \
                  title TEXT NOT NULL, \
                  caseid INTEGER NOT NULL, \
                  followupdate INTEGER, \
                  FOREIGN KEY(caseid) REFERENCES ews_case(id)\
                );",
                "DROP TABLE ews_item;");

ews_migration!(1470499981, CreateNotes, "create notes table",
               "CREATE TABLE ews_note(\
                  id INTEGER PRIMARY KEY, \
                  itemid INTEGER NOT NULL, \
                  body TEXT, \
                  modifieddate INTEGER NOT NULL, \
                  FOREIGN KEY(itemid) REFERENCES ews_item(id)\
                );",
                "DROP TABLE ews_note;");

macro_rules! register {
    ( $migrator:expr, $( $migration:ident ),* ) => {
        $(
            $migrator.register(Box::new($migration));
         )*
    }
}

pub fn run(conn: &rusqlite::Connection) -> Result<(), MigrationError> {
    let adapter = SqliteAdapter::new(conn);
    adapter.setup_schema();

    let mut migrator = schemamama::Migrator::new(adapter);
    register!(migrator, CreateUsers, CreateCases, CreateItems, CreateNotes);
    migrator.up(None)
}
