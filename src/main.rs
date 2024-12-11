mod structs;
mod fwatch;

fn main() {
    let arguments: structs::Args = argh::from_env();
    fwatch::watch_for_file(arguments)

}
