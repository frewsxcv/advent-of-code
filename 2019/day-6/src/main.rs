use std::collections::{HashMap, HashSet};

const CENTER_OF_MASS: &str = "COM";
const INPUT: &str = include_str!("input.txt");

type SpaceObject = String;

#[derive(Debug)]
struct SpaceMap {
    space_objects: HashSet<SpaceObject>,
    // { orbiter: orbitee }
    orbital_relationships_map: HashMap<SpaceObject, SpaceObject>,
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
            .map(|s| self.num_orbits_from_center_of_mass(s))
            .sum()
    }

    fn num_orbits_from_center_of_mass(&self, space_object: &SpaceObject) -> u32 {
        if space_object == CENTER_OF_MASS {
            0
        } else {
            1 + self.num_orbits_from_center_of_mass(
                &self.orbital_relationships_map[space_object]
            )
        }
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
