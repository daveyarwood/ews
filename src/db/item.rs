use rusqlite;
use time::Timespec;
use util;

pub struct Item {
    pub id: i64,
    pub title: String,
    pub case_id: i64,
    pub follow_up_date: Option<Timespec>
}

pub fn due_today(conn: &rusqlite::Connection, user_id: i64)
    -> Result<Vec<Item>, rusqlite::Error> {
    let tomorrow = util::midnight_tomorrow();

    let mut stmt = try!(conn.prepare(
        "SELECT i.id, i.title, i.caseid, i.followupdate
           FROM ews_item i
     INNER JOIN ews_case c ON c.id = i.caseid
          WHERE c.userid = :user_id
            AND c.closeddate IS NULL
            AND i.followupdate IS NOT NULL
            AND i.followupdate < :tomorrow"));

    let rows = try!(stmt.query_map(&[&user_id, &tomorrow], |row| {
        Item {
            id: row.get(0),
            title: row.get(1),
            case_id: row.get(2),
            follow_up_date: row.get(3),
        }
    }));

    rows.collect()
}
