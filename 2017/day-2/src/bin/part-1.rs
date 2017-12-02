static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", calculate_checksum(INPUT));
}

fn calculate_checksum(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let v =
                line.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
            v.iter().max().unwrap()
                - v.iter().min().unwrap()
        })
        .sum()
}

#[test]
fn example_tests() {
    let input = "5 1 9 5
                 7 5 3
                 2 4 6 8";
    assert_eq!(18, calculate_checksum(input));
}
