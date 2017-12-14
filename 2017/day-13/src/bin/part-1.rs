static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", calculate_severity(INPUT));
}

fn parse_line(line: &str) -> (u16, u16) {
    let mut iter = line.split(": ").map(|n| n.parse::<u16>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn calculate_severity(input: &str) -> u16 {
    let mut severity = 0;
    for (depth, range) in input.trim().lines().map(parse_line) {
        if scanner_position(depth, range) == 0 {
            severity += depth * range;
        }
    }
    severity
}

fn scanner_position(time: u16, range: u16) -> u16 {
    (0..range - 1)
        .chain((1..range).rev())
        .cycle()
        .skip(time as usize)
        .next()
        .unwrap()
}

#[test]
fn test_examples() {
    let input = "0: 3
1: 2
4: 4
6: 4";
    assert_eq!(24, calculate_severity(input));
}
