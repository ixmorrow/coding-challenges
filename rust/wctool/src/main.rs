use std::env;
use std::process;
use wctool;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = wctool::parse_config(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("In file {}", file_path);

    if let Err(e) = wctool::run(file_path) {
        println!("Application error: {e}");
        process::exit(1);
    }
}