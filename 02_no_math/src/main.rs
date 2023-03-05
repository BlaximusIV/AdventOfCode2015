use std:: { error::Error, fs };

mod wrapping_sqr_ft;
mod package_parser;
mod measure_ribbon;

fn main() {
    let input = import_input("input.txt").unwrap();

    let packages = package_parser::parse_package_strings(&input);

    let total_sqr_feet = wrapping_sqr_ft::calculate_total_sqr_ft(&packages);
    let ribbon_length = measure_ribbon::calculate_required_ribbon(&packages);

    println!("The total required wrapping paper is: {} ft squared", total_sqr_feet);
    println!("The total required ribbon is: {} ft", ribbon_length);
}

fn import_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    Ok(input)
}
