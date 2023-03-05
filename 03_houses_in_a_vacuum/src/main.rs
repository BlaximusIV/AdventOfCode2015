use std:: { error::Error, fs };
use std::time::{Instant};

mod route_plotter;

fn main() {
    let now = Instant::now();

    // instructions are only a single line
    let input = import_input("input.txt").unwrap();

    let unique_house_count = route_plotter::find_house_count(input.lines().nth(0).unwrap());

    println!("{} unique homes visited.", unique_house_count);

    println!("Elapsed time: {}", now.elapsed().as_millis())
}

fn import_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    Ok(input)
}
