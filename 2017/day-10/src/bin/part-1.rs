static INPUT: [usize; 16] = [
    46, 41, 212, 83,
    1, 255, 157, 65,
    139, 52, 39, 254,
    2, 86, 0, 204,
];

fn main() {
    let mut skip_size = 0;
    let mut curr_pos = 0;
    let mut list = (0..256).collect::<Vec<u16>>();
    for rev_len in INPUT.iter() {
        step(&mut list, curr_pos, *rev_len);
        curr_pos += rev_len + skip_size;
        skip_size += 1;
    }
    println!("solution: {}", list[0] * list[1]);
}

fn step(list: &mut [u16], curr_pos: usize, rev_len: usize) {
    let mut new_vals = list.iter()
        .cloned()
        .cycle()
        .skip(curr_pos)
        .take(rev_len)
        .collect::<Vec<_>>();
    new_vals.reverse();
    let list_len = list.len();
    for (i, val) in new_vals.into_iter().enumerate() {
        list[(curr_pos + i) % list_len] = val;
    }
}

#[test]
fn test_step() {
    let mut a = [0, 1, 2, 3];
    step(&mut a, 0, 3, 0);
    assert_eq!([2, 1, 0, 3], a);

    let mut a = [0, 1, 2, 3];
    step(&mut a, 3, 3, 0);
    assert_eq!([0, 3, 2, 1], a);
}
