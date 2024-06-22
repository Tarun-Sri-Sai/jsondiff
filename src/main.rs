mod input;
mod json_diff;

fn main() {
    let (file1, file2) = input::read_arguments();

    input::assert_file_exists(&file1);
    input::assert_file_exists(&file2);

    let json_data1 = input::get_data_from_inputs(&file1);
    let json_data2 = input::get_data_from_inputs(&file2);

    json_diff::show(&json_data1, &json_data2);
}
