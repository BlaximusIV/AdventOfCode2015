use std::collections::HashMap;

pub fn find_house_count(directions: &str) -> usize {
    let mut visited_coordinates: HashMap<[i32; 2], usize> = HashMap::new();
    let mut current_santa_coordinate: [i32; 2] = [0,0];
    let mut currernt_robo_coordinate: [i32; 2] = [0,0];

    visited_coordinates.insert(current_santa_coordinate, 1);
    
    let mut santa_turn: bool = true;
    for c in directions.chars() {
        let mut next_coordinate: [i32; 2];

        if santa_turn {
            next_coordinate = find_next_coordinate(current_santa_coordinate, c);
            current_santa_coordinate = next_coordinate;
        } else {
            next_coordinate = find_next_coordinate(currernt_robo_coordinate, c);
            currernt_robo_coordinate = next_coordinate;
        }

        insert_visited_coordinate(&next_coordinate, &mut visited_coordinates);
        santa_turn = !santa_turn;
    }

    visited_coordinates.keys().count()
}

fn find_next_coordinate(current: [i32; 2], direction: char) -> [i32; 2] {
    let mut next_coordinate = [current[0], current[1]];

    match direction {
        '>' => next_coordinate[0] += 1,
        '<' => next_coordinate[0] -= 1,
        '^' => next_coordinate[1] += 1,
        'v' => next_coordinate[1] -= 1,
        _ => panic!("Unrecognized character detected in direction. Direction: {}", direction)
    }

    next_coordinate
} 

fn insert_visited_coordinate(coordinate: &[i32; 2], coordinate_map: &mut HashMap<[i32;2], usize>){
    
    if !coordinate_map.contains_key(coordinate) {
        coordinate_map.insert(*coordinate, 1);
    } else {
        *coordinate_map.entry(*coordinate).or_insert(1) += 1;
    }

}

#[test]
fn finds_next_coordinate(){
    assert_eq!(find_next_coordinate([0,0], '>'), [1,0]);
    assert_eq!(find_next_coordinate([0,0], '<'), [-1,0]);
    assert_eq!(find_next_coordinate([0,0], '^'), [0,1]);
    assert_eq!(find_next_coordinate([0,0], 'v'), [0,-1]);
}

#[test]
fn array_equality(){
    assert_eq!([0,1], [0,1]);
}