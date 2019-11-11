#[macro_use] extern crate lazy_static;

mod fetch;
mod clean;
mod summary;

fn start() -> i32 {
    let ku_username = match std::env::args().skip(1).next() {
        Some(username) => username,
        None => {
            eprintln!("Please provide your ku username as the first argument.");
            return 1;
        },
    };

    let cals = match fetch::fetch(ku_username.as_str()) {
        Ok(cals) => cals,
        Err(err) => {
            eprintln!("Failed to fetch calendar: {}.", err);
            return 1;
        },
    };

    let calendar = clean::clean(cals);

    match calendar.print() {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Failure during printing: {}", err);
            return 1;
        },
    }

    0
}

fn main() {
    std::process::exit(start());
}
