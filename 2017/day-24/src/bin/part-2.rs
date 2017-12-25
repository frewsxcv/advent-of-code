#[derive(Copy, Clone, Debug)]
struct Component {
    left_pins: u16,
    right_pins: u16,
}

impl Component {
    fn from_str(s: &str) -> Self {
        let mut iter = s.trim().split("/");
        Component {
            left_pins: iter.next().unwrap().parse().unwrap(),
            right_pins: iter.next().unwrap().parse().unwrap(),
        }
    }

    fn score(self) -> u16 {
        self.left_pins + self.right_pins
    }
}

#[derive(Debug, Clone, Default)]
struct Components(Vec<Component>);

impl Components {
    fn from_str(s: &str) -> Self {
        Components(
            s.trim()
                .lines()
                .map(|l| Component::from_str(l))
                .collect()
        )
    }

    fn score(&self) -> u16 {
        self.0.iter().map(|n| n.score()).sum()
    }
}

fn recurse(prev_pins: u16, remaining: Components, bridge: Components) -> Components {
    let mut highest_length = 0;
    let mut best_score = 0;
    let mut best_bridge = None;
    for (index, component) in remaining.0.iter().enumerate() {
        let curr_bridge = if component.left_pins == prev_pins {
            let mut remaining_clone = remaining.clone();
            let mut bridge_clone = bridge.clone();
            let removed = remaining_clone.0.remove(index);
            bridge_clone.0.push(removed);
            recurse(removed.right_pins, remaining_clone, bridge_clone)
        } else if component.right_pins == prev_pins {
            let mut remaining_clone = remaining.clone();
            let mut bridge_clone = bridge.clone();
            let removed = remaining_clone.0.remove(index);
            bridge_clone.0.push(removed);
            recurse(removed.left_pins, remaining_clone, bridge_clone)
        } else {
            continue;
        };
        if curr_bridge.0.len() > highest_length || curr_bridge.score() > best_score {
            highest_length = curr_bridge.0.len();
            best_score = curr_bridge.score();
            best_bridge = Some(curr_bridge);
        }
    }
    best_bridge.unwrap_or(bridge)
}

fn main() {
    let bridge = recurse(
        0,
        Components::from_str(include_str!("../input")),
        Components(vec![]),
    );
    println!("solution: {}", bridge.score());
}
