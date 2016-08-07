extern crate ews;

use ews::db;

fn main() {
    db::run_migrations().unwrap();

    println!("done");
}
