use std:: { error::Error, fs };
use regex::Regex;

const GRID_SIZE:usize = 1000;

#[derive(Debug, PartialEq)]
enum InstructionType {
    On,
    Off,
    Toggle
}

struct Coordinate {
    x: usize,
    y: usize
}

struct Instruction {
    i_type: InstructionType,
    start: Coordinate,
    end: Coordinate
}

fn main() {
    let instructions = import_input("Input.txt").unwrap();
    
    let mut grid = create_grid();
    
    for instruction in instructions {
        execute(&mut grid, instruction)
    }

    let count = count_lights(grid);

    println!("On lights: {}", count);
}

fn import_input(file_name: &'static str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;
    
    let reg: Regex = Regex::new(r"([a-z\s]+)\s([\d]+,[\d]+) through ([\d]+,[\d]+)").unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let captures = reg.captures(line).unwrap();

        let i_type: InstructionType = 
            if captures[1].contains("on") { InstructionType::On }
            else if captures[1].contains("off") { InstructionType::Off }
            else { InstructionType::Toggle };

        instructions.push(Instruction { 
            i_type, 
            start: parse_coord(&captures[2]), 
            end: parse_coord(&captures[3]) });
    }

    Ok(instructions)
}

fn create_grid() -> [[bool;GRID_SIZE]; GRID_SIZE] {
    let row:[bool;GRID_SIZE] = [false; GRID_SIZE];
    let grid:[[bool;GRID_SIZE]; GRID_SIZE] = [row; GRID_SIZE];

    grid
}

fn parse_coord(text: &str) -> Coordinate {
    let yx_str = text.split(",").collect::<Vec<&str>>();
    Coordinate { x: yx_str[0].parse::<usize>().unwrap(), y: yx_str[1].parse::<usize>().unwrap() }
}

fn execute(grid: &mut [[bool;GRID_SIZE]; GRID_SIZE], instruction: Instruction) {
    for i in instruction.start.y..instruction.end.y + 1{
        for j in instruction.start.x..instruction.end.x + 1 {
            grid[i][j] = 
                if instruction.i_type == InstructionType::On { true }
                else if instruction.i_type == InstructionType::Off { false }
                else { !grid[i][j] }
        }
    }
}

fn count_lights(grid: [[bool;GRID_SIZE]; GRID_SIZE]) -> usize {
    let mut on_count: usize = 0;

    for i in 0..GRID_SIZE{
        for j in 0..GRID_SIZE {
            if grid[i][j] {
                on_count += 1;
            }
        }
    }

    on_count
}

#[test]
fn correct_counts() {
    let mut grid = create_grid();

    grid[234][12] = true;
    grid[224][12] = true;
    grid[214][12] = true;
    grid[204][12] = true;
    grid[274][12] = true;
    grid[999][999] = true;

    let count = count_lights(grid);
    assert_eq!(count, 6);
}

#[test]
fn correct_on() {
    let mut grid = create_grid();

    let instruction = Instruction {
        i_type: InstructionType::On,
        start: Coordinate { x: 0, y: 0 },
        end: Coordinate { x: 999, y: 999 }
    };

    execute(&mut grid, instruction);

    let count = count_lights(grid);
    assert_eq!(count, 1_000_000);
}