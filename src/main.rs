use std::process;
mod input;
mod json_diff;

fn main() {
    let (file1, file2) = input::read_arguments();

    input::assert_file_exists(&file1);
    input::assert_file_exists(&file2);

    let json_data1 = match input::get_data_from_tui(&file1) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error getting JSON data from TUI for {}: {}", file1, e);
            process::exit(0);
        }
    };
    let json_data2 = match input::get_data_from_tui(&file2) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error getting JSON data from TUI for {}: {}", file2, e);
            process::exit(0);
        }
    };

    json_diff::show(&json_data1, &json_data2);
}
