use regex::Regex;

pub fn get_good_string_count(input: &str) -> usize {
    let mut count: usize = 0;

    let vowel_regex = get_vowel_count_regex();
    let naughty_string_regex = get_naughty_strings_regex();

    for line in input.lines() {
        let contains_vowels = vowel_regex.captures_iter(line).count() >= 3;
        let repeats_chars = contains_repeating_chars(line);
        let contains_naughty_strings = naughty_string_regex.is_match(line);

        if contains_vowels && repeats_chars && !contains_naughty_strings {
            count += 1;
        }
    }

    count
}

fn get_vowel_count_regex() -> Regex {
    let regex = Regex::new(r"([aeiou])").unwrap();

    regex
}

fn get_naughty_strings_regex() -> Regex {
    let regex = Regex::new(r"ab|cd|pq|xy").unwrap();

    regex
}

fn contains_repeating_chars(line: &str) -> bool {
    let mut contains_repeating_chars: bool = false;

    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() - 1{
        if chars[i] == chars[i + 1] {
            contains_repeating_chars = true;
        }
    }

    contains_repeating_chars
}

#[test]
fn count_regex_counts() {
    let regex = get_vowel_count_regex();

    let mut count = regex.captures_iter("ugknbfddgicrmopn").count();

    assert_eq!(count, 3);

    count = regex.captures_iter("abcde").count();
    assert_eq!(count, 2);
}

#[test]
fn finds_repeating_strings() {
    assert_eq!(contains_repeating_chars("aa"), true);
    assert_eq!(contains_repeating_chars("ba"), false);
    assert_eq!(contains_repeating_chars("werapsoii"), true);
    assert_eq!(contains_repeating_chars("apdieksla"), false);
}

#[test]
fn naughty_strings_accurate(){
    let regex = get_naughty_strings_regex();

    assert_eq!(regex.is_match("ab"), true);
    assert_eq!(regex.is_match("cd"), true);
    assert_eq!(regex.is_match("pq"), true);
    assert_eq!(regex.is_match("xy"), true);
    assert_eq!(regex.is_match("dslajd"), false);
}