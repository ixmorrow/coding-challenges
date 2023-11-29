use std::fs;
use std::error::Error;

pub fn parse_config(args: &[String]) -> Result<&str, &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments - Program expects a file path argument.");
    }

    let file_path = &args[1];

    Ok(file_path)
}

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let bytes = contents.len();

    println!("With text:\n{contents}");
    println!("Bytes: {bytes}");
    Ok(())
}