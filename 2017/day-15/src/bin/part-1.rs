const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const DIVISOR: u64 = 2147483647;
const BIT_MASK: u64 = 0b1111_1111_1111_1111;

fn main() {
    let mut a = 783;
    let mut b = 325;
    let mut count = 0;

    for _ in 0..40_000_000 {
        next_value(&mut a, FACTOR_A);
        next_value(&mut b, FACTOR_B);
        if (a & BIT_MASK) == (b & BIT_MASK) {
            count += 1;
        }
    }

    println!("solution: {}", count);
}

fn next_value(prev_value: &mut u64, factor: u64) {
    *prev_value = *prev_value * factor % DIVISOR;
}
