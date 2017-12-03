extern crate day_3;

use day_3::Square;

fn main() {
    let value =
        (0..std::usize::MAX)
            .map(|n| Square::from_square_num(n).value())
            .filter(|v| *v > 347991)
            .next()
            .unwrap();
    println!("solution: {}", value);
}
