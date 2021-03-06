use query::Query;
use rusqlite;
use std;
use time;
use time::Timespec;
use util;

#[derive(Clone, Debug)]
pub struct Case {
    pub id: i64,
    pub title: String,
    pub user_id: i64,
    pub opened_date: Timespec,
    pub closed_date: Option<Timespec>
}

pub fn print_cases(cases: Vec<Case>) {
    println!("ID\tTITLE\tOPEN FOR");
    for case in cases {
        let open_for = if case.closed_date.is_some() {
            "".to_string()
        } else {
            format!("{} days", util::age_in_days(case.opened_date))
        };
        println!("{}\t{}\t{}", case.id, case.title, open_for);
    }
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

pub fn followups_due_today(conn: &rusqlite::Connection, user_id: i64)
    -> Result<Vec<Case>, rusqlite::Error> {
    let tomorrow = util::midnight_tomorrow();

    let mut stmt = try!(conn.prepare(
        "SELECT c.id, c.title, c.openeddate
           FROM ews_case c
     INNER JOIN ews_item i ON c.id = i.caseid
          WHERE c.userid = :user_id
            AND c.closeddate IS NULL
            AND i.followupdate IS NOT NULL
            AND i.followupdate < :tomorrow"));

    let rows = try!(stmt.query_map(&[&user_id, &tomorrow], |row| {
        Case {
            id: row.get(0),
            user_id: user_id,
            title: row.get(1),
            opened_date: row.get(2),
            closed_date: None
        }
    }));

    rows.collect()
}

pub fn no_action_in_days(days: i64, conn: &rusqlite::Connection, user_id: i64)
    -> Result<Vec<Case>, rusqlite::Error> {
    let n_days_ago = util::days_ago(days);

    let mut stmt = try!(conn.prepare(
        "SELECT c.id, c.title, c.openeddate
           FROM ews_case c
          WHERE c.userid = :user_id
            AND c.closeddate IS NULL
            AND (SELECT COUNT(i.id)
                   FROM ews_item i
                  WHERE i.caseid = c.id
                    AND i.createddate > :n_days_ago)
                = 0"));

    let rows = try!(stmt.query_map(&[&user_id, &n_days_ago], |row| {
        Case {
            id: row.get(0),
            user_id: user_id,
            title: row.get(1),
            opened_date: row.get(2),
            closed_date: None
        }
    }));

    rows.collect()
}

fn choose_case(conn: &rusqlite::Connection, user_id: i64, cases: Vec<Case>,
               open_cases_only: bool)
    -> Result<Option<Case>, rusqlite::Error> {
    println!("Your query returned multiple results:\n");

    print_cases(cases);
    println!("");

    let query = util::prompt(
        "Please enter a case ID or part of the title: ");

    find_case(conn, user_id, Query::new(query), open_cases_only)
}

pub fn find_case(conn: &rusqlite::Connection, user_id: i64, query: Query,
                 open_cases_only: bool)
    -> Result<Option<Case>, rusqlite::Error> {
    match query {
        Query::Id(id) => {
            let mut stmt = try!(conn.prepare(
                    "SELECT id, title, openeddate, closeddate
                       FROM ews_case
                      WHERE userid = :user_id
                        AND id = :id"));
            let rows = try!(stmt.query_map(&[&user_id, &id], |row| {
                Case {
                    id: row.get(0),
                    title: row.get(1),
                    user_id: user_id,
                    opened_date: row.get(2),
                    closed_date: row.get(3)
                }
            }));

            let mut cases = Vec::<Case>::new();
            for row in rows {
                let row = try!(row);
                if !open_cases_only || row.closed_date.is_none() {
                    cases.push(row);
                }
            }

            match cases.len() {
                0 => Ok(None),
                1 => Ok(Some(cases[0].clone())),
                _ => { panic!("Found > 1 case in the db with the same id!"); }
            }

        },
        Query::SearchString(search) => {
            let mut stmt = try!(conn.prepare(
                    "SELECT id, title, openeddate, closeddate
                       FROM ews_case
                      WHERE userid = :user_id
                        AND title like :search"));
            let rows = try!(stmt.query_map(&[&user_id,
                                             &format!("%{}%", &search)],
                                           |row| {
                Case {
                    id: row.get(0),
                    title: row.get(1),
                    user_id: user_id,
                    opened_date: row.get(2),
                    closed_date: row.get(3)
                }
            }));

            let mut cases = Vec::<Case>::new();
            for row in rows {
                let row = try!(row);
                if !open_cases_only || row.closed_date.is_none() {
                    cases.push(row);
                }
            }

            match cases.len() {
                0 => Ok(None),
                1 => Ok(Some(cases[0].clone())),
                _ => choose_case(&conn, user_id, cases, open_cases_only)
            }

        }
    }
}

pub fn print_summary(conn: &rusqlite::Connection, user_id: i64, query: Query)
    -> Result<(), rusqlite::Error> {
    let query_type = match query {
        Query::Id(_) => "ID",
        Query::SearchString(_) => "search string"
    };

    match try!(find_case(conn, user_id, query, false)) {
        None => {
            println!("No case found matching that {}.", query_type);
            std::process::exit(1);
        },
        Some(case) => {
            println!("ID:\t{}", case.id);
            println!("Title:\t{}", case.title);
            println!("Open?:\t{}", case.closed_date.is_none());
        }
    }

    Ok(())
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

