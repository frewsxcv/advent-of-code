extern crate day_5;

use day_5::parse;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", determine_num_steps(INPUT));
}

fn determine_num_steps(input: &str) -> u64 {
    let mut parsed = parse(input);
    let mut curr_idx = 0;
    let mut num_steps = 0;

    while let Some(idx_offset) = parsed.get(curr_idx).map(|n| *n) {
        parsed[curr_idx] += if idx_offset >= 3 { -1 } else { 1 };
        curr_idx = (curr_idx as i64 + idx_offset) as usize;
        num_steps += 1;
    }

    num_steps
}

#[test]
fn test_determine_num_steps() {
    assert_eq!(
        determine_num_steps("0\n3\n0\n1\n-3"),
        10,
    )
}
