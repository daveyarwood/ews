use query::Query;
use rusqlite;
use time;
use time::Timespec;
use util;

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

pub fn find_case(conn: &rusqlite::Connection, user_id: i64, query: Query)
    -> Result<Option<Case>, rusqlite::Error> {
    // FIXME
    Ok(None)
}

pub fn create_case(conn: &rusqlite::Connection, title: String, user_id: i64, opened_date: Timespec)
    -> Result<i64, rusqlite::Error> {
    conn.execute_named(
      "INSERT INTO ews_case (title, userid, openeddate)
       VALUES (:title, :userid, :openeddate)",
       &[(":title", &title),
         (":userid", &user_id),
         (":openeddate", &opened_date)]
    ).map(|_| conn.last_insert_rowid())
}

pub fn open_case(conn: &rusqlite::Connection, title: Option<&str>, user_id: i64)
    -> Result<i64, rusqlite::Error> {
    let title = match title {
        Some(title) => title.to_string(),
        None => { util::prompt("Please enter a title for this case: ") }
    };

    create_case(conn, title, user_id, time::get_time())
}

pub fn close_case(conn: &rusqlite::Connection, case_id: i64)
    -> Result<(), rusqlite::Error> {
    conn.execute_named(
      "UPDATE ews_case
          SET closeddate = :closed_date
        WHERE id = :case_id",
       &[(":case_id", &case_id), (":closed_date", &time::get_time())]
    ).map(|_| ())
}

