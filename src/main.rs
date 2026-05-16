mod structs;
mod fwatch;

use std::process;

fn main() {
    let arguments: structs::Args = argh::from_env();
    process::exit(fwatch::watch_for_file(&arguments));
}
