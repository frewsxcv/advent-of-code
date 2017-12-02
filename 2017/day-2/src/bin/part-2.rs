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
    for (i, smaller_num) in nums.iter().enumerate() {
        for larger_num in &nums[(i + 1)..] {
            if larger_num % smaller_num == 0 {
                return larger_num / smaller_num;
            }
        }
    }
    panic!("Could not determine score for line")
}

fn tokenize_line<'a>(line: &'a str) -> Box<Iterator<Item=usize> + 'a> {
    Box::new(
        line.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
    )
}

#[test]
fn example_tests() {
    let input = "5 9 2 8
                 9 4 7 3
                 3 8 6 5";
    assert_eq!(9, calculate_checksum(input));
}
