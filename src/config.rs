use std::env::home_dir;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn ews_home_dir() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(".ews");
    path
}

pub fn ews_home_dir_exists() -> bool {
    match fs::metadata(ews_home_dir()) {
        Err(_)   => false,
        Ok(path) => path.is_dir()
    }
}

pub fn create_ews_home_dir() -> io::Result<()> {
    if !ews_home_dir_exists() {
        fs::create_dir(ews_home_dir()).unwrap();
    }

    Ok(())
}

pub fn db_file() -> PathBuf {
    let mut path = ews_home_dir();
    path.push("ews.db");
    path
}

