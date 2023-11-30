use std::env;
use std::process;
use wctool;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = wctool::parse_config(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Config:\n{}", config);

    if let Err(e) = wctool::run(config.clone()) {
        println!("Application error: {e}");
        process::exit(1);
    }
}