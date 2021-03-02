pub fn get_good_string_count(input: &str) -> usize {
    let mut count: usize = 0;

    for line in input.lines() {
        if contains_repeating_pairs(line) && contains_sandwich(line) {
            count += 1;
        }
    }

    count
}

fn contains_repeating_pairs(line: &str) -> bool {
    let mut has_repeating_pairs = false;

    // Absolutely terrible performance. Needs optimization.
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() - 3 {
        for j in i + 2 .. chars.len() - 1 {
            if chars[i] == chars[j] && chars[i + 1] == chars[j + 1] {
                has_repeating_pairs = true;
                break;
            }
        }
        if has_repeating_pairs {
            break;
        }
    }

    has_repeating_pairs
}

fn contains_sandwich(line: &str) -> bool {
    let mut has_sandwich = false;

    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            has_sandwich = true;
            break;
        }
    }

    has_sandwich
}

#[test]
fn finds_repeating_pairs() {
    assert_eq!(contains_repeating_pairs("xyxy"), true);
    assert_eq!(contains_repeating_pairs("aaa"), false);
    assert_eq!(contains_repeating_pairs("aabcdefgaa"), true);
    assert_eq!(contains_repeating_pairs("abcdefgh"), false);
}

#[test]
fn finds_sandwiches() {
    assert_eq!(contains_sandwich("aaa"), true);
    assert_eq!(contains_sandwich("xyx"), true);
    assert_eq!(contains_sandwich("abcdefeghi"), true);
    assert_eq!(contains_sandwich("abcde"), false);
}