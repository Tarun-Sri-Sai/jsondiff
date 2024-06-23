use serde_json::Value;
use std::fs;
use std::process;

pub fn parse_file(path: &str) -> Value {
    let file_data = match fs::read_to_string(path) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error reading {}: {}", path, err);
            process::exit(0);
        }
    };
    return match serde_json::from_str(&file_data) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            process::exit(0);
        }
    };
}

pub fn parse_path(input: &str) -> Vec<String> {
    let trimmed = input.trim();
    let mut keys = Vec::new();
    let mut current_key = String::new();
    let mut chars = trimmed.chars().peekable();
    let mut inside_quotes = false;

    while let Some(&c) = chars.peek() {
        match c {
            '\\' => {
                chars.next();
                if let Some(&next_c) = chars.peek() {
                    current_key.push(next_c);
                    chars.next();
                }
            }
            '"' => {
                inside_quotes = !inside_quotes;
                current_key.push(c);
                chars.next();
            }
            '.' => {
                if inside_quotes {
                    current_key.push(c);
                    chars.next();
                } else {
                    if !current_key.is_empty() {
                        keys.push(current_key.clone());
                        current_key.clear();
                    }
                    chars.next();
                }
            }
            _ => {
                current_key.push(c);
                chars.next();
            }
        }
    }
    if !current_key.is_empty() {
        keys.push(current_key);
    }
    return keys;
}

pub fn navigate(file_data: Value, keys: Vec<String>) -> Value {
    let mut data = &file_data;

    for key in keys {
        let array_index: Option<usize> = match key.parse::<usize>() {
            Ok(res) => Some(res),
            Err(_) => None,
        };

        let inner_data = match array_index {
            None => {
                if key.len() < 2 {
                    return data.clone();
                }
                match data.get(&key[1..key.len() - 1]) {
                    None => return data.clone(),
                    Some(property) => property,
                }
            }
            Some(res) => match data.get(res) {
                None => return data.clone(),
                Some(res) => res,
            },
        };

        data = inner_data;
    }

    return data.clone();
}
