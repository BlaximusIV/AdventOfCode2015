use std:: { error::Error, fs };

mod route_plotter;

fn main() {
    // instructions are only a single line
    let input = import_input("input.txt").unwrap();

    let unique_house_count = route_plotter::find_house_count(input.lines().nth(0).unwrap());

    
}

fn import_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    Ok(input)
}
