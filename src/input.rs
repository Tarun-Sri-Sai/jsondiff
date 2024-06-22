use serde_json::Value;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process;

mod json_parser;

pub fn read_arguments() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <json file 1> <json file 2>", args[0]);
        process::exit(0);
    }

    return (args[1].clone(), args[2].clone());
}

pub fn assert_file_exists(path: &str) {
    if !Path::new(&path).is_file() {
        eprintln!("{} is not a file", path);
        process::exit(0);
    }
}

fn prompt(message: String) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}

pub fn get_data_from_inputs(file: &str) -> Value {
    prompt(format!("Enter JSON path for {}: ", file));
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return json_parser::navigate(
        json_parser::parse_file(file),
        json_parser::parse_path(&input),
    );
}
