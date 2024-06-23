use clap::{Arg, ArgMatches, Command};
use console;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_json::{self, to_string_pretty, Value};
use std::error::Error;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color as TuiColor, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
mod json_parser;

pub fn read_arguments() -> ArgMatches {
    return Command::new("jsondiff")
        .arg(
            Arg::new("file1")
                .help("First JSON file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("file2")
                .help("Second JSON file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("input_mode")
                .short('i')
                .long("input")
                .value_name("INPUT_MODE")
                .default_value("default"),
        )
        .get_matches();
}

pub fn assert_file_exists(path: &str) {
    if !Path::new(&path).is_file() {
        eprintln!("{} is not a file", path);
        process::exit(0);
    }
}

fn display_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    file: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut input = String::new();
    let mut json_data: Value = json_parser::parse_file(file);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(5)].as_ref())
                .split(f.size());

            let input_paragraph = Paragraph::new(input.as_ref())
                .style(Style::default().fg(TuiColor::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("JSON Path for {}", file)),
                );

            let data_paragraph = Paragraph::new(
                to_string_pretty(&json_data).unwrap_or_else(|_| "Invalid JSON data".to_string()),
            )
            .style(Style::default().fg(TuiColor::Green))
            .block(Block::default().borders(Borders::ALL).title("JSON Data"));

            f.render_widget(input_paragraph, chunks[0]);
            f.render_widget(data_paragraph, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    json_data = json_parser::navigate(
                        json_parser::parse_file(file),
                        json_parser::parse_path(&input),
                    );
                }
                KeyCode::Backspace => {
                    input.pop();
                    json_data = json_parser::navigate(
                        json_parser::parse_file(file),
                        json_parser::parse_path(&input),
                    );
                }
                _ => {}
            }
        }
    }

    return Ok(json_data);
}

pub fn get_data_from_tui(file: &str) -> Result<Value, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = display_tui(&mut terminal, file);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    return res;
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

fn print_quickview_data(json_data: &Value) {
    let pretty_json = serde_json::to_string_pretty(&json_data)
        .unwrap_or_else(|_| "Invalid JSON data".to_string());
    let lines: Vec<&str> = pretty_json.lines().take(10).collect();
    for line in &lines {
        println!("{}", line);
    }

    let all_lines: Vec<&str> = pretty_json.lines().collect();
    if all_lines.len() > lines.len() {
        println!("...more");
    }
}

pub fn get_data_from_quickview(file: &str) -> Value {
    let mut input = String::new();
    let mut json_data = json_parser::parse_file(file);

    print_quickview_data(&json_data);

    loop {
        prompt(format!("\x1B[2J\x1B[1;1H"));
        print_quickview_data(&json_data);
        prompt(format!("Enter JSON path for {}: {}", file, input));

        if let Ok(key) = console::Term::stdout().read_char() {
            match key {
                '\n' => {
                    println!("");
                    break;
                }
                _ => {
                    input.push(key);
                    json_data = json_parser::navigate(
                        json_parser::parse_file(file),
                        json_parser::parse_path(&input),
                    );
                }
            }
        }
    }

    prompt(format!("\x1B[2J\x1B[1;1H"));
    return json_data;
}
