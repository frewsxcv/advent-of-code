extern crate day_1;

use day_1::ascii_byte_to_val;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", captcha_solver(INPUT));
}

fn captcha_solver(captcha: &str) -> usize {
    let bytes = captcha.as_bytes(); // assume string is ASCII

    (0..bytes.len())
        .filter_map(|idx| {
            if bytes[idx] == bytes[((idx + (bytes.len() / 2)) % bytes.len())] {
                Some(ascii_byte_to_val(bytes[idx]))
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn provided_tests() {
    assert_eq!(6, captcha_solver("1212"));
    assert_eq!(0, captcha_solver("1221"));
    assert_eq!(4, captcha_solver("123425"));
    assert_eq!(12, captcha_solver("123123"));
    assert_eq!(4, captcha_solver("12131415"));
}

#[test]
fn empty_test() {
    assert_eq!(0, captcha_solver(""));
}
