use std::collections::HashMap;

const NUM_PLAYERS: u16 = 430;
const LAST_MARBLE: u32 = 7_158_800;

fn main() {
    let mut circle = Vec::with_capacity(LAST_MARBLE as usize);
    circle.insert(0, 0);
    let mut curr_marble_idx = 1usize;
    let mut scores = HashMap::<u16, u32>::with_capacity(NUM_PLAYERS as usize);

    for (next_marble_value, curr_player) in (1..=LAST_MARBLE).zip((0..NUM_PLAYERS).cycle()) {
        if next_marble_value % 23 == 0 {
            let score = scores.entry(curr_player).or_insert(0);
            curr_marble_idx = curr_marble_idx
                .checked_sub(7)
                .unwrap_or(circle.len() - 7 + curr_marble_idx)
                % circle.len();
            *score += circle.remove(curr_marble_idx) + next_marble_value;
        } else {
            let next_marble_idx = (curr_marble_idx + 1) % circle.len() + 1;
            circle.insert(next_marble_idx, next_marble_value);
            curr_marble_idx = next_marble_idx;
        }
    }

    println!("{}", scores.values().max().unwrap());
}
