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
    let num_unique_words = line.trim().split_whitespace().collect::<HashSet<_>>().len();
    num_words == num_unique_words
}

#[test]
fn test_is_valid_line() {
    assert!(is_valid_line("aa bb cc dd ee"));
    assert!(!is_valid_line("aa bb cc dd aa"));
    assert!(is_valid_line("aa bb cc dd aaa"));
}
