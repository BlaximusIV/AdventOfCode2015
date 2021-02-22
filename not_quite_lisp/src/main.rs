use std:: { error::Error, fs };
mod find_floor;
mod find_basement_command;

fn main() {
    let input = import_input("input.txt").unwrap();

    find_floor::find_target_floor(&input);
    find_basement_command::get_basement_direction(input.lines().nth(0).unwrap());
}

fn import_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    Ok(input)
}