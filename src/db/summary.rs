use db::{case};
use rusqlite;

pub fn print_summary(conn: &rusqlite::Connection, user_id: i64)
    -> Result<(), rusqlite::Error> {
    let open_cases = try!(case::all_open_cases(conn, user_id));
    println!("Open cases:\t{}", open_cases.len());

    let due_today = try!(case::followups_due_today(conn, user_id));
    println!("Follow-ups due today:\t{}", due_today.len());

    let inactive = try!(case::no_action_in_days(30, conn, user_id));
    println!("No action in 30 days:\t{}", inactive.len());

    Ok(())
}
