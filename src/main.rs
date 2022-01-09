use rust_grep::run;
use std::process;

fn main() {
    if let Err(err) = run() {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
