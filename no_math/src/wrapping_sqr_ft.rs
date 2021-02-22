pub fn calculate_total_sqr_ft(input: String) -> usize {
    let mut total: usize = 0;

    for line in input.lines(){
        let (length, width, height) = parse_package(line);
        total += calculate_package_paper(length, width, height);
    }

    total
}

fn parse_package(line: &str) -> (usize, usize, usize) {
    let dimensions = line.split("x").collect::<Vec<&str>>();
    let dimensions: Vec<usize> = dimensions.iter().map(|x| x.parse::<usize>().unwrap()).collect();

    (dimensions[0], dimensions[1], dimensions[2])
}

fn calculate_package_paper(length: usize, width: usize, height: usize) -> usize {
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
fn parses_package(){
    assert_eq!(parse_package("1x2x3"), (1,2,3));
    assert_eq!(parse_package("23x2x4"), (23, 2, 4));
}

#[test]
fn gets_smallest_side(){
    assert_eq!(get_smallest_side([12,34,2]), 2);
    assert_eq!(get_smallest_side([2,1,3]), 1);
    assert_eq!(get_smallest_side([4,4,5]), 4);
}

#[test]
fn calculates_correctly(){
    assert_eq!(calculate_package_paper(2, 3, 4), 58);
    assert_eq!(calculate_package_paper(1, 1, 10), 43);
}