// https://www.youtube.com/watch?v=AqXTZo6o34s

fn main() {
    let mut b = 106_500;
    let c = 123_500;
    let mut h = 0;

    loop {
        let mut f = true;
        for d in 2..b {
            if b % d == 0 {
                f = false;
                break;
            }
        }
        if !f {
            h += 1;
        }
        if b == c {
            break;
        }
        b += 17;
    }

    println!("solution: {}", h);
}
