use std::process;
mod input;
mod json_diff;

fn main() {
    let args = input::read_arguments();

    let file1 = args.get_one::<String>("file1").unwrap();
    input::assert_file_exists(&file1);

    let file2 = args.get_one::<String>("file2").unwrap();
    input::assert_file_exists(&file2);

    let input_mode = args.get_one::<String>("input_mode").unwrap();
    let json_data1;
    let json_data2;

    match input_mode.as_str() {
        "tui" | "t" => {
            json_data1 = match input::get_data_from_tui(file1) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("Error getting JSON data from TUI for {}: {}", file1, e);
                    process::exit(0);
                }
            };
            json_data2 = match input::get_data_from_tui(file2) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("Error getting JSON data from TUI for {}: {}", file2, e);
                    process::exit(0);
                }
            };
        }
        "quickview" | "q" => {
            json_data1 = input::get_data_from_quickview(file1);
            json_data2 = input::get_data_from_quickview(file2);
        }
        _ => {
            json_data1 = input::get_data_from_inputs(file1);
            json_data2 = input::get_data_from_inputs(file2);
        }
    };

    json_diff::show(&json_data1, &json_data2);
}
