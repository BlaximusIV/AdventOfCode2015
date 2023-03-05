use std:: { error::Error, fs, time::Instant };
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
    let now = Instant::now();

    let instructions = import_input("Input.txt").unwrap();
    
    // Part 1
    let mut grid = create_bool_grid();
    for instruction in &instructions {
        execute_bool(&mut grid, &instruction)
    }

    let count = count_lights(grid);
    println!("On lights: {}", count);

    // Part 2
    let mut grid = create_int_grid();
    for instruction in instructions {
        execute_int(&mut grid, &instruction)
    }

    let level = count_light_levels(&grid);
    println!("Combined light levels: {}", level);

    println!("Elapsed time: {}ms", now.elapsed().as_millis());
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

fn parse_coord(text: &str) -> Coordinate {
    let yx_str = text.split(",").collect::<Vec<&str>>();
    Coordinate { x: yx_str[0].parse::<usize>().unwrap(), y: yx_str[1].parse::<usize>().unwrap() }
}

fn create_bool_grid() -> [[bool;GRID_SIZE]; GRID_SIZE] {
    let row:[bool;GRID_SIZE] = [false; GRID_SIZE];
    let grid = [row; GRID_SIZE];

    grid
}

fn create_int_grid() -> Vec<Vec<usize>> {
    let row = vec![0;GRID_SIZE];
    let grid = vec![row; GRID_SIZE];

    grid
}

fn execute_bool(grid: &mut [[bool;GRID_SIZE]; GRID_SIZE], instruction: &Instruction) {
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

fn execute_int(grid: &mut Vec<Vec<usize>>, instruction: &Instruction) {
    for i in instruction.start.y..instruction.end.y + 1{
        for j in instruction.start.x..instruction.end.x + 1 {
                if instruction.i_type == InstructionType::On { grid[i][j] += 1 }
                else if instruction.i_type == InstructionType::Off { 
                    if grid[i][j] > 0 {
                        grid[i][j] -= 1
                    }
                 }
                else { grid[i][j] += 2 }
        }
    }
}

fn count_light_levels(grid: &Vec<Vec<usize>>) -> usize {
    let mut level_count: usize = 0;

    for i in 0..GRID_SIZE{
        for j in 0..GRID_SIZE {
            level_count += grid[i][j];
        }
    }

    level_count
}

#[test]
fn correct_on_counts() {
    let mut grid = create_bool_grid();

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
    let mut grid = create_bool_grid();

    let instruction = Instruction {
        i_type: InstructionType::On,
        start: Coordinate { x: 0, y: 0 },
        end: Coordinate { x: 999, y: 999 }
    };

    execute_bool(&mut grid, &instruction);

    let count = count_lights(grid);
    assert_eq!(count, 1_000_000);
}

#[test]
fn correct_level_counts() {
    let mut grid = create_int_grid();

    grid[234][12] = 45;
    grid[224][12] = 3;
    grid[214][12] = 4;
    grid[204][12] = 4;
    grid[274][12] = 49;
    grid[999][999] = 12;

    assert_eq!(count_light_levels(&grid), 117);
}

#[test]
fn correct_levels() {
    let mut grid = create_int_grid();
    
    let mut instruction = Instruction {
        i_type: InstructionType::Toggle,
        start: Coordinate { x: 0, y: 0 },
        end: Coordinate { x: 999, y: 999 }
    };

    execute_int(&mut grid, &instruction);

    assert_eq!(count_light_levels(&grid), 2_000_000);

    instruction.i_type = InstructionType::Off;
    execute_int(&mut grid, &instruction);

    assert_eq!(count_light_levels(&grid), 1_000_000);

    instruction.i_type = InstructionType::On;
    execute_int(&mut grid, &instruction);

    assert_eq!(count_light_levels(&grid), 2_000_000);
}