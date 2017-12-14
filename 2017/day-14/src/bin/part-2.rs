extern crate petgraph;
extern crate hex;

fn main() {
    let grid = build_grid();

    let mut graph = petgraph::graphmap::UnGraphMap::new();

    for y in 0..128 {
        for x in 0..128 {
            if grid[y][x] {
                graph.add_node((x, y));
                if x > 0 && grid[y][x - 1] {
                    graph.add_edge((x, y), (x - 1, y), ());
                }
                if y > 0 && grid[y - 1][x] {
                    graph.add_edge((x, y), (x, y - 1), ());
                }
            }
        }
    }

    let num_connected_components =
        petgraph::algo::connected_components(&graph);
    println!("solution: {}", num_connected_components);
}

fn build_grid() -> Vec<Vec<bool>> {
    let mut grid = Vec::with_capacity(128);
    for i in 0..128 {
        let mut inner_v = Vec::with_capacity(128);
        let n = format!("hwlqcszp-{}", i);
        let hash = calculate_hash(n.as_bytes());
        let decoded = hex::decode(hash).unwrap();
        for byte_ in decoded {
            for _ in 0..byte_.leading_zeros() {
                inner_v.push(false);
            }
            if byte_ != 0 {
                for n in format!("{:b}", byte_).bytes() {
                    inner_v.push(if n == b'1' { true } else { false });
                }
            }
        }
        assert_eq!(128, inner_v.len());
        grid.push(inner_v);
    }
    grid
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

