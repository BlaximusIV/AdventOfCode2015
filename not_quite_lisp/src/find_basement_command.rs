pub fn get_basement_direction(input_text: &str) {
    let instruction_number = find_instruction_number(input_text);
    println!("The {}th instruction leads to the basement floor.", instruction_number);
}

fn find_instruction_number(input_text: &str) -> usize {
    let mut increment : i32 = 0;
    
    for c in 0..input_text.chars().count() {
        if input_text.chars().nth(c).unwrap() == '(' {
            increment += 1;
        } else {
            increment -= 1;
        }

        if increment == -1 {
            return c + 1;
        }
    }

    panic!("Basement level not reached!");
}

#[test]
fn finds_basement(){
    assert_eq!(find_instruction_number(")"), 1);
    assert_eq!(find_instruction_number("()())"), 5);
}