use std::collections::HashSet;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", num_valid_lines(INPUT));
}

fn num_valid_lines(lines: &str) -> usize {
    lines.trim().lines().filter(|l| is_valid_line(l)).count()
}

fn is_valid_line(line: &str) -> bool {
    let num_words = line.trim().split_whitespace().count();
    let num_unique_anagrams =
        line.trim()
            .split_whitespace()
            .map(|s| {
                let mut b = s.as_bytes().to_owned();
                b.sort();
                b
            })
            .collect::<HashSet<_>>()
            .len();
    num_words == num_unique_anagrams
}

#[test]
fn test_is_valid_line() {
    assert!(is_valid_line("abcde fghij"));
    assert!(!is_valid_line("abcde xyz ecdab"));
    assert!(is_valid_line("a ab abc abd abf abj"));
    assert!(is_valid_line("iiii oiii ooii oooi oooo"));
    assert!(!is_valid_line("oiii ioii iioi iiio"));
}
