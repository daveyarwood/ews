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
    pub closed_date: Timespec
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

