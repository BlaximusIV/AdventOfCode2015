pub fn calculate_required_ribbon(packages: &Vec<Vec<usize>>) -> usize {
    let mut ribbon_length: usize = 0;

    for package in packages {
        ribbon_length += measure_ribbon(&package);
    }

    ribbon_length
}

fn measure_ribbon(package: &Vec<usize>) -> usize {
    let wrap_ribbon_length = (2 * package[0]) + (2 * package[1]);
    let bow_ribbon_length = package[0] * package[1] * package[2];

    wrap_ribbon_length + bow_ribbon_length
}

#[test]
fn measures_ribbbon(){
    assert_eq!(measure_ribbon(&vec![2,3,4]), 34);
    assert_eq!(measure_ribbon(&vec![1,1,10]), 14);
}
