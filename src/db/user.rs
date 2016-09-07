use rusqlite;
use std::io;
use std::io::Write;

pub struct User {
    pub id: i64,
    pub name: String
}

fn create_user(conn: &rusqlite::Connection, name: String)
    -> Result<i64, rusqlite::Error> {
    conn.execute_named(
      "INSERT INTO ews_user (name) VALUES (:name)", &[(":name", &name)]
    ).map(|_| conn.last_insert_rowid())
}

pub fn current_user(conn: &rusqlite::Connection) -> Option<User> {
    let result: Result<User, rusqlite::Error> = conn.query_row_and_then(
      "SELECT id, name FROM ews_user
       WHERE  id in (
         SELECT userid FROM ews_state
         WHERE userid != 0
         LIMIT 1
       );",
       &[],
       |row| Ok(User { id: row.get(0), name: row.get(1) })
    );

    result.ok()
}

pub fn set_current_user(conn: &rusqlite::Connection, user_id: i64)
    -> Result<(), rusqlite::Error> {
    conn.execute_named(
      "UPDATE ews_state SET userid = :userid", &[(":userid", &user_id)]
    ).map(|_| ())
}

pub fn create_new_user(conn: &rusqlite::Connection)
    -> Result<(), rusqlite::Error> {
    print!("Please enter your name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    match create_user(conn, name.trim().to_string()) {
        Ok(id) => set_current_user(conn, id),
        Err(e) => Err(e)
    }
}

