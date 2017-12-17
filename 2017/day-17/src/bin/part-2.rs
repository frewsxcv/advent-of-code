const INPUT: usize = 376;

fn main() {
    let mut cur_pos = 0;
    let mut val_after_zero = 0;
    for i in 1..50_000_000 {
        if cur_pos == 0 {
            val_after_zero = i;
        }
        cur_pos = (cur_pos + 1 + INPUT) % (i as usize + 1);
    }
    println!("{}", val_after_zero);
}
