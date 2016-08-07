use std::env::home_dir;
use std::path::PathBuf;

pub fn ews_home_dir() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(".ews");
    path
}

pub fn ews_db_file() -> PathBuf {
    let mut path = ews_home_dir();
    path.push("ews.db");
    path
}

