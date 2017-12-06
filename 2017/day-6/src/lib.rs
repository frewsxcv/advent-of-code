use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

pub struct Blocks(pub Vec<usize>);

impl Blocks {
    pub fn redistrib_once(&mut self) {
        let index = self.next_index_for_redistrib();
        let val = mem::replace(&mut self.0[index], 0);
        let len = self.0.len();
        for i in 0..val {
            self.0[(index + i + 1) % len] += 1
        }
    }

    pub fn next_index_for_redistrib(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, val)| val)
            .map(|(index, _)| index)
            .unwrap()
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

#[test]
fn test_next_index_for_redistrib() {
    assert_eq!(2, Blocks(vec![0, 2, 7, 0]).next_index_for_redistrib());
    assert_eq!(3, Blocks(vec![0, 1, 2, 7, 7, 4]).next_index_for_redistrib());
}

#[test]
fn test_redistrib_once() {
    let mut b = Blocks(vec![0, 2, 7, 0]);
    b.redistrib_once();
    assert_eq!(vec![2, 4, 1, 2], b.0);
}
