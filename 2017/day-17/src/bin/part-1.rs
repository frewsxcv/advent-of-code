const INPUT: usize = 376;

fn main() {
    let mut buf: Vec<u32> = vec![0];
    let mut cur_pos = 0;

    for i in 1..2018 {
        buf.insert(cur_pos + 1, i);
        cur_pos = (cur_pos + 1 + INPUT) % buf.len();
    }

    let pos = buf.iter().position(|n| *n == 2017).unwrap();
    println!("{:?}", buf[pos + 1]);
}
