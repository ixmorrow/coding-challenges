use std::error::Error;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::process;
use std::fmt;

pub struct FileData {
    bytes: usize,
    lines: u32,
    chars: usize,
    words: usize
}

impl FileData {
    fn build(bytes: &usize, lines: &u32, chars: &usize, words: &usize) -> FileData {
        let file_bytes =  bytes.clone();
        let file_lines = lines.clone();
        let words = words.clone();
        let file_chars = chars.clone();

        FileData { bytes: file_bytes, lines: file_lines, chars: file_chars, words: words }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    query_param: Option<String>,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. Expected at least 2 arguments.");
        }

        let file_path = args.get(1).cloned().ok_or("File path not provided")?;

        let query_param = match args.get(2) {
            Some(param) => Some(param.clone()),
            None => None,
        };

        Ok(Config{ query_param, file_path })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Query Param: {:?}, File Path: {}", self.query_param, self.file_path)
    }
}

pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments - Program expects a file path argument.");
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    Ok(config)
}

pub fn run(config: Config) -> Result<FileData, Box<dyn Error>> {
    let mut number_of_lines: u32 = 0;
    let mut bytes: usize = 0;
    let mut chars: usize = 0;
    let mut words: usize = 0;

    if let Ok(lines) = read_lines(&config.file_path) {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                bytes += line.len();
                number_of_lines+=1;
                chars += line.chars().count();
                words += word_count(&line);
            }
        }
    }

    let file_data = FileData::build(&bytes, &number_of_lines, &chars, &words);

    match config.query_param.as_deref().unwrap_or("") {
        "-c" => println!("{0} {1}", file_data.bytes, &config.file_path),
        "-l" => println!("{0} {1}", file_data.lines, &config.file_path),
        "-w" => println!("{0} {1}", file_data.words, &config.file_path),
        "-m" => println!("{0} {1}", file_data.chars, &config.file_path),
        _query_param => println!("{0} {1} {2} {3} {4}", file_data.bytes, file_data.lines, file_data.chars, file_data.words, &config.file_path),
    }    
    Ok(file_data)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn word_count(line: &str) -> usize {
    line.split_whitespace().count()
}