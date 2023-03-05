pub fn calculate_total_sqr_ft(packages: &Vec<Vec<usize>>) -> usize {
    let mut total: usize = 0;

    for package in packages {
        total += calculate_package_paper(package);
    }

    total
}

fn calculate_package_paper(package: &Vec<usize>) -> usize {
    let (length, width, height) = (package[0], package[1], package[2]);

    let length_width = length * width;
    let width_height = width * height;
    let height_length = height * length;

    let smallest_side = get_smallest_side([length_width, width_height, height_length]);

    // Surface area + indicated extra margin
    (2 * length_width) + (2 * width_height) + (2 * height_length) + smallest_side
}

fn get_smallest_side(sides: [usize; 3]) -> usize {
    let mut smallest_side = sides[0];

    if sides[1] < smallest_side {
        smallest_side = sides[1]; 
    }

    if sides[2] < smallest_side {
        smallest_side = sides[2];
    }

    smallest_side
}

#[test]
fn gets_smallest_side(){
    assert_eq!(get_smallest_side([12,34,2]), 2);
    assert_eq!(get_smallest_side([2,1,3]), 1);
    assert_eq!(get_smallest_side([4,4,5]), 4);
}

#[test]
fn calculates_correctly(){
    assert_eq!(calculate_package_paper(&vec![2, 3, 4]), 58);
    assert_eq!(calculate_package_paper(&vec![1, 1, 10]), 43);
}