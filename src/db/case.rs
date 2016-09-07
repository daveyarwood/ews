use rusqlite;
use std::io;
use std::io::Write;
use time;
use time::Timespec;

pub struct Case {
    pub id: i64,
    pub title: String,
    pub user_id: i64,
    pub opened_date: Timespec,
    pub closed_date: Option<Timespec>
}

pub fn all_open_cases(conn: &rusqlite::Connection, user_id: i64)
    -> Result<Vec<Case>, rusqlite::Error> {
    let mut stmt = try!(conn.prepare("SELECT id, title, openeddate
                             FROM ews_case
                             WHERE userid = :user_id
                             AND closeddate IS NULL"));

    let rows = try!(stmt.query_map(&[&user_id], |row| {
        Case {
            id: row.get(0),
            title: row.get(1),
            user_id: user_id,
            opened_date: row.get(2),
            closed_date: None
        }
    }));

    rows.collect()
}

pub fn create_case(conn: &rusqlite::Connection, title: String, user_id: i64)
    -> Result<i64, rusqlite::Error> {
    conn.execute_named(
      "INSERT INTO ews_case (title, userid, openeddate)
       VALUES (:title, :userid, :openeddate)",
       &[(":title", &title),
         (":userid", &user_id),
         (":openeddate", &time::get_time())]
    ).map(|_| conn.last_insert_rowid())
}

pub fn create_new_case(conn: &rusqlite::Connection, title: Option<&str>, user_id: i64)
    -> Result<i64, rusqlite::Error> {
    let title = match title {
        Some(title) => title.to_string(),
        None => {
            print!("Please enter a title for this case: ");
            io::stdout().flush().unwrap();
            let mut title = String::new();
            io::stdin().read_line(&mut title).unwrap();
            title.trim().to_string()
        }
    };

    create_case(conn, title, user_id)
}

