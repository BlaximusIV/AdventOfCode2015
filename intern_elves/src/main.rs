use std:: { error::Error, fs };

mod line_counter;

fn main() {
    let input = import_input("input.txt").unwrap();

    let good_count = line_counter::get_good_string_count(&input);

    println!("Good line count: {}", good_count);
}

fn import_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    Ok(input)
}
