use std:: { error::Error, fs };

fn main() {
    let input = import_input("input.txt").unwrap();

    let mut target_floor: i32 = 0;

    for line in input.lines() {
        target_floor += get_floor_increment(line);
    }

    print!("The target floor is: {}", target_floor);
}

fn import_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    Ok(input)
}

fn get_floor_increment(line: &str) -> i32 {
    let mut increment : i32 = 0;

    for c in line.chars() {
        if c == '(' {
            increment += 1;
        } else {
            increment -= 1;
        }
    }

    increment
}

#[test]
fn correctly_counts(){
    assert_eq!(get_floor_increment("(())"), 0);
    assert_eq!(get_floor_increment("()()"), 0);
    assert_eq!(get_floor_increment("((("), 3);
    assert_eq!(get_floor_increment("(()(()("), 3);
    assert_eq!(get_floor_increment("))((((("), 3);
    assert_eq!(get_floor_increment("())"), -1);
    assert_eq!(get_floor_increment("))("), -1);
    assert_eq!(get_floor_increment(")))"), -3);
    assert_eq!(get_floor_increment(")())())"), -3);
}