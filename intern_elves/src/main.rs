use std:: { error::Error, fs };

mod line_counter;
mod line_counter_2;

fn main() {
    let input = import_input("input.txt").unwrap();

    let original_good_count = line_counter::get_good_string_count(&input);
    println!("Original Good line count: {}", original_good_count);

    let new_good_count = line_counter_2::get_good_string_count(&input);
    println!("New Good line count: {}", new_good_count);
}

fn import_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    Ok(input)
}
