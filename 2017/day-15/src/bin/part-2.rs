#![feature(conservative_impl_trait)]

use std::iter;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

const FILTER_A: u64 = 4;
const FILTER_B: u64 = 8;

const DIVISOR: u64 = 2147483647;

const BIT_MASK: u64 = 0b1111_1111_1111_1111;

fn main() {
    let a = 783;
    let b = 325;

    let count: usize =
        values_iter(a, FACTOR_A, FILTER_A)
            .zip(values_iter(b, FACTOR_B, FILTER_B))
            .filter(|&(a, b)| (a & BIT_MASK) == (b & BIT_MASK))
            .count();

    println!("solution: {}", count);
}

fn values_iter(init_value: u64, factor: u64, filter: u64) -> impl Iterator<Item=u64> {
    iter::repeat(()).scan(init_value, move |value, _| {
        loop {
            *value = *value * factor % DIVISOR;
            if *value % filter == 0 {
                break Some(*value);
            }
        }
    })
    .take(5_000_000)
}
