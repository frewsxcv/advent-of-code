use std::collections::{HashMap, HashSet};

const CENTER_OF_MASS: &str = "COM";
const INPUT: &str = include_str!("input.txt");

type SpaceObject = String;
// { orbiter: orbitee }
type OrbitalRelationshipsMap = HashMap<SpaceObject, SpaceObject>;

#[derive(Debug)]
struct SpaceMap {
    space_objects: HashSet<SpaceObject>,
    orbital_relationships_map: OrbitalRelationshipsMap,
}

impl SpaceMap {
    fn from_input() -> Self {
        let space_objects = INPUT
            .lines()
            .map(OrbitalRelationship::from_line)
            .flat_map(|o| vec![o.orbiter.to_string(), o.orbitee.to_string()].into_iter())
            .collect();
        let orbital_relationships_map = INPUT
            .lines()
            .map(OrbitalRelationship::from_line)
            .map(|o| (o.orbiter, o.orbitee))
            .collect();
        SpaceMap {
            orbital_relationships_map,
            space_objects,
        }
    }

    fn num_orbits(&self) -> u32 {
        self.space_objects
            .iter()
            .map(|mut space_object| {
                let mut count = 0;
                while space_object != CENTER_OF_MASS {
                    space_object = &self.orbital_relationships_map[space_object];
                    count += 1;
                }
                count
            })
            .sum()
    }
}

// "2YQ)3JS" -> { orbitee: "2YQ", orbiter: "3JS" }
#[derive(Debug)]
struct OrbitalRelationship {
    orbitee: SpaceObject,
    orbiter: SpaceObject,
}

impl OrbitalRelationship {
    fn from_line(s: &str) -> Self {
        let mut split = s.split(")");
        OrbitalRelationship {
            orbitee: split.next().unwrap().into(),
            orbiter: split.next().unwrap().into(),
        }
    }
}

fn main() {
    println!("part 1: {}", SpaceMap::from_input().num_orbits());
}
