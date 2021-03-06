extern crate day_6;

use day_6::Blocks;
use std::collections::HashSet;

fn main() {
    let mut b = Blocks(vec![4, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5]);
    println!("solution: {:?}", redistrib(&mut b));
}

fn redistrib(blocks: &mut Blocks) -> usize {
    let mut set = HashSet::new();
    let mut cycles = 0;
    loop {
        let hash = blocks.hash();
        if set.contains(&hash) {
            break cycles;
        }
        cycles += 1;
        set.insert(hash);
        blocks.redistrib_once();
    }
}

#[test]
fn test_redistrib() {
    let mut b = Blocks(vec![0, 2, 7, 0]);
    assert_eq!(5, redistrib(&mut b));
    assert_eq!(vec![2, 4, 1, 2], b.0);
}
