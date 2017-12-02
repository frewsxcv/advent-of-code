static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", calculate_checksum(INPUT));
}

fn calculate_checksum(input: &str) -> usize {
    input.lines().map(score_line).sum()
}

fn score_line(line: &str) -> usize {
    tokenize_line(line).max().unwrap()
        - tokenize_line(line).min().unwrap()
}

fn tokenize_line<'a>(line: &'a str) -> Box<Iterator<Item=usize> + 'a> {
    Box::new(
        line.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
    )
}

#[test]
fn example_tests() {
    let input = "5 1 9 5
                 7 5 3
                 2 4 6 8";
    assert_eq!(18, calculate_checksum(input));
}
