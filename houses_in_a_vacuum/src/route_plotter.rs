use std::collections::HashMap;

pub fn find_house_count(directions: &str) -> usize {
    let mut visited_coordinates: Vec<(i32, Vec<i32>)> = vec![(0,vec![0])];
    let mut current_coordinate: [i32; 2] = [0,0];

    

    1
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

fn insert_visited_coordinate(coordinate: &[i32; 2], coordinate_map: &mut Vec<(i32, Vec<i32>)>){
    
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

#[test]
fn hashmap_keys(){
    let mut hash: HashMap<[i32; 2], Vec<i32>> = HashMap::new();

    
}