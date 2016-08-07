use rusqlite;
use schemamama::Migrator;
use schemamama_rusqlite::{SqliteAdapter, SqliteMigration, SqliteMigrationError};

struct CreateUsers;
migration!(CreateUsers, 1470499978, "create users table");

struct CreateCases;
migration!(CreateCases, 1470499979, "create cases table");

struct CreateItems;
migration!(CreateItems, 1470499980, "create items table");

struct CreateNotes;
migration!(CreateNotes, 1470499981, "create notes table");

impl SqliteMigration for CreateUsers {
    fn up(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        conn.execute("CREATE TABLE ews_user(\
                        id INTEGER PRIMARY KEY, \
                        name TEXT NOT NULL\
                      );", &[]).map(|_| ())
    }

    fn down(&self, connection: &rusqlite::Connection) -> rusqlite::Result<()> {
        connection.execute("DROP TABLE ews_user;", &[]).map(|_| ())
    }
}

impl SqliteMigration for CreateCases {
    fn up(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        conn.execute("CREATE TABLE ews_case(\
                        id INTEGER PRIMARY KEY, \
                        title TEXT NOT NULL, \
                        userid INTEGER NOT NULL, \
                        openeddate INTEGER NOT NULL, \
                        closeddate INTEGER, \
                        FOREIGN KEY(userid) REFERENCES ews_user(id)\
                      );", &[]).map(|_| ())
    }

    fn down(&self, connection: &rusqlite::Connection) -> rusqlite::Result<()> {
        connection.execute("DROP TABLE ews_case;", &[]).map(|_| ())
    }
}

impl SqliteMigration for CreateItems {
    fn up(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        conn.execute("CREATE TABLE ews_item(\
                        id INTEGER PRIMARY KEY, \
                        title TEXT NOT NULL, \
                        caseid INTEGER NOT NULL, \
                        followupdate INTEGER, \
                        FOREIGN KEY(caseid) REFERENCES ews_case(id)\
                      );", &[]).map(|_| ())
    }

    fn down(&self, connection: &rusqlite::Connection) -> rusqlite::Result<()> {
        connection.execute("DROP TABLE ews_item;", &[]).map(|_| ())
    }
}

impl SqliteMigration for CreateNotes {
    fn up(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        conn.execute("CREATE TABLE ews_note(\
                        id INTEGER PRIMARY KEY, \
                        itemid INTEGER NOT NULL, \
                        body TEXT, \
                        modifieddate INTEGER NOT NULL, \
                        FOREIGN KEY(itemid) REFERENCES ews_item(id)\
                      );", &[]).map(|_| ())
    }

    fn down(&self, connection: &rusqlite::Connection) -> rusqlite::Result<()> {
        connection.execute("DROP TABLE ews_note;", &[]).map(|_| ())
    }
}

pub fn run(conn: &rusqlite::Connection) -> Result<(), SqliteMigrationError> {
    let adapter = SqliteAdapter::new(conn);
    adapter.setup_schema();

    let mut migrator = Migrator::new(adapter);

    let migrations: Vec<Box<SqliteMigration>> = vec![
        Box::new(CreateUsers),
        Box::new(CreateCases),
        Box::new(CreateItems),
        Box::new(CreateNotes),
    ];

    for migration in migrations {
        migrator.register(migration);
    }

    // bogus number that will be higher than any epoch timestamp
    migrator.up(1999999999)
}
