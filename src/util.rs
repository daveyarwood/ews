use time;
use time::Timespec;

pub fn age_in_days(then: Timespec) -> i64 {
    let now = time::get_time();
    (now - then).num_days()
}
