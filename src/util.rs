use std::io;
use std::io::Write;
use time;
use time::Timespec;

pub fn age_in_days(then: Timespec) -> i64 {
    let now = time::get_time();
    (now - then).num_days()
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
