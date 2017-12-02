extern crate day_2;

use day_2::tokenize_line;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", calculate_checksum(INPUT));
}

fn calculate_checksum(input: &str) -> usize {
    input.lines().map(score_line).sum()
}

fn score_line(line: &str) -> usize {
    let mut nums = tokenize_line(line).collect::<Vec<_>>();
    nums.sort();
    nums.iter()
        .enumerate()
        .flat_map(|(i, smaller_num)| {
            std::iter::repeat(smaller_num).zip(&nums[(i + 1)..])
        })
        .filter_map(|(smaller_num, larger_num)| if larger_num % smaller_num ==
            0
        {
            Some(larger_num / smaller_num)
        } else {
            None
        })
        .next()
        .expect("Could not determine score for line")
}

#[test]
fn example_tests() {
    let input = "5 9 2 8
                 9 4 7 3
                 3 8 6 5";
    assert_eq!(9, calculate_checksum(input));
}
