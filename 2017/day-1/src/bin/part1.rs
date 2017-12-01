extern crate day_1;

use day_1::ascii_byte_to_val;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", captcha_solver(INPUT));
}

fn captcha_solver(captcha: &str) -> usize {
    let captcha_bytes = captcha.as_bytes(); // assume string is ASCII

    let mut sum = captcha_bytes
        .windows(2)
        .filter_map(|window| if window[0] == window[1] {
            Some(ascii_byte_to_val(window[0]))
        } else {
            None
        })
        .sum();

    if captcha_bytes.len() > 2 && captcha_bytes.first() == captcha_bytes.last() {
        sum += ascii_byte_to_val(captcha_bytes[0]);
    }

    sum
}

#[test]
fn provided_tests() {
    assert_eq!(3, captcha_solver("1122"));
    assert_eq!(4, captcha_solver("1111"));
    assert_eq!(0, captcha_solver("1234"));
    assert_eq!(9, captcha_solver("91212129"));
}

#[test]
fn empty_test() {
    assert_eq!(0, captcha_solver(""));
}

#[test]
fn basic_tests() {
    assert_eq!(1, captcha_solver("11"));
    assert_eq!(0, captcha_solver("10"));
}
