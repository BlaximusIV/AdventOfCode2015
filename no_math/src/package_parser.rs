pub fn parse_package_strings(input: &String) -> Vec<Vec<usize>>{
    let mut packages: Vec<Vec<usize>> = vec![];
    
    for line in input.lines(){
        let package = parse_package(line);
        packages.push(package);
    }

    packages
}

fn parse_package(line: &str) -> Vec<usize> {
    let dimensions = line.split("x").collect::<Vec<&str>>();
    let mut dimensions: Vec<usize> = dimensions.iter().map(|x| x.parse::<usize>().unwrap()).collect();

    dimensions.sort();
    dimensions
}

#[test]
fn parses_package(){
    assert_eq!(parse_package("1x2x3"), [1,2,3]);
    assert_eq!(parse_package("23x2x4"), [2,4,23]);
}