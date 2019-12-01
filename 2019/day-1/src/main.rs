const INPUT: &str = include_str!("../input.txt");

fn fuel_required(mass: f32) -> f32 {
    (mass / 3.0).floor() - 2.0
}

fn input_lines() -> impl Iterator<Item = f32> {
    INPUT
        .lines()
        .map(|s| s.parse::<f32>().unwrap())
}

fn part_1_answer() -> f32 {
    input_lines().map(fuel_required).sum()
}

fn main() {
    println!("part 1 answer: {}", part_1_answer())
}
