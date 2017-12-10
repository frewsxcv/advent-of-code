static INPUT: &[u8] = b"46,41,212,83,1,255,157,65,139,52,39,254,2,86,0,204";
static EXTRA: &[u8] = &[17, 31, 73, 47, 23];

fn main() {
    let mut skip_size = 0usize;
    let mut curr_pos = 0usize;
    let mut list = (0..256).collect::<Vec<u32>>();

    let mut lengths = INPUT.to_owned();
    lengths.extend_from_slice(EXTRA);

    lengths.iter()
        .cycle()
        .take(lengths.len() * 64)
        .for_each(|rev_len| {
            step(&mut list[..], curr_pos, *rev_len as usize);
            curr_pos += (*rev_len as usize) + skip_size;
            skip_size += 1;
        });

    let mut dense_hash = [0; 16];
    for (i, n) in list.into_iter().enumerate() {
        dense_hash[i / 16] ^= n;
    }

    for n in dense_hash.iter() {
        let f = format!("{:x}", n);
        if f.len() == 1 {
            print!("0");
        }
        print!("{}", f);
    }
    println!("");
}

fn step(list: &mut [u32], curr_pos: usize, rev_len: usize) {
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
