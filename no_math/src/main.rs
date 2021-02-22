use std:: { error::Error, fs };

mod wrapping_sqr_ft;

fn main() {
    let input = import_input("input.txt").unwrap();

    let total_sqr_feet = wrapping_sqr_ft::calculate_total_sqr_ft(input);

    println!("The total required wrapping paper is: {} ft squared", total_sqr_feet);
}

fn import_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    Ok(input)
}
