use regex::Regex;

mod md5_hasher;

fn main() {
    const KEY: &str = "ckczppom";
    let mut target_number = 0;

    let re: Regex = Regex::new(r"^0{6}[0-9a-f]*").unwrap();
    let mut hash_found = false;

    while !hash_found {
        let hash = md5_hasher::md5_utf8(&(KEY.to_owned() + &target_number.to_string()));

        if re.is_match(&hash) {
            hash_found = true;
            break;
        }

        target_number += 1;
    }

    println!("Target found: {}", target_number);
}