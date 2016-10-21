use std::io;
use std::io::Write;
use time;
use time::Duration;
use time::Timespec;

pub fn age_in_days(then: Timespec) -> i64 {
    let now = time::get_time();
    (now - then).num_days()
}

pub fn midnight_tomorrow() -> Timespec {
    let now = time::at(time::get_time());

    let tomorrow_string = format!("{}-{}-{}T00:00", 1900 + now.tm_year,
                                                    now.tm_mon + 1,
                                                    now.tm_mday + 1);

    let tomorrow_utc = time::strptime(&tomorrow_string, "%Y-%m-%dT%H:%M")
                       .unwrap()
                       .to_local();

    let offset = time::Duration::seconds(tomorrow_utc.tm_utcoff as i64);

    let tomorrow = (tomorrow_utc - offset).to_local();

    tomorrow.to_timespec()
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
