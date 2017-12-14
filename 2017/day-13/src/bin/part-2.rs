static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", find_offset(INPUT));
}

fn find_offset(input: &str) -> u64 {
    let lines = input.trim().lines().map(parse_line).collect::<Vec<(u64, u64)>>();
    for offset in 0..std::u64::MAX {
        if !will_get_caught(&lines, offset) {
            return offset;
        }
    }
    unreachable!()
}

fn parse_line(line: &str) -> (u64, u64) {
    let mut iter = line.split(": ").map(|n| n.parse::<u64>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn will_get_caught(input: &[(u64, u64)], offset: u64) -> bool {
    for &(depth, range) in input {
        if scanner_position_is_zero(depth + offset, range) {
            return true;
        }
    }
    false
}

fn scanner_position_is_zero(time: u64, range: u64) -> bool {
    time % ((range - 1) * 2) == 0
}

#[test]
fn test_examples() {
    let input = "0: 3
1: 2
4: 4
6: 4";
    assert_eq!(10, find_offset(input));
}
