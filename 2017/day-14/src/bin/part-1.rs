extern crate hex;

fn main() {
    let mut sum_ones: u32 = 0;
    for i in 0..128 {
        let n = format!("hwlqcszp-{}", i);
        let hash = calculate_hash(n.as_bytes());
        let decoded = hex::decode(hash).unwrap();
        let num_zeros: u32 = decoded.into_iter().map(|b| b.count_ones()).sum();
        sum_ones += num_zeros;
    }
    println!("solution: {}", sum_ones);
}

fn calculate_hash(input: &[u8]) -> String {
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

    let mut skip_size = 0usize;
    let mut curr_pos = 0usize;
    let mut list = (0..256).collect::<Vec<u32>>();
    static EXTRA: &[u8] = &[17, 31, 73, 47, 23];

    let mut lengths = input.to_owned();
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

    let mut out = String::new();
    for n in dense_hash.iter() {
        let f = format!("{:x}", n);
        if f.len() == 1 {
            out.push('0');
        }
        out.push_str(&f);
    }
    out
}

