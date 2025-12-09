use rustaoc2025::get_input;
use std::collections::hash_map::Values;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3 {
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() != 3 {
            return Err(format!("invalid point format: {}", s));
        }

        let x = parts[0]
            .parse::<i64>()
            .map_err(|_| format!("invalid x in {}", s))?;
        let y = parts[1]
            .parse::<i64>()
            .map_err(|_| format!("invalid y in {}", s))?;
        let z = parts[2]
            .parse::<i64>()
            .map_err(|_| format!("invalid z in {}", s))?;

        Ok(Point3 { x, y, z })
    }

    pub fn euclidean(&self, other: &Point3) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

fn parse_points(input: &str) -> Result<Vec<Point3>, String> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty()) // skip blank lines
        .map(Point3::parse)
        .collect()
}

fn solve(input: &str) -> usize {
    let junction_boxes: Vec<Point3> = parse_points(input).unwrap();
    // for each point find its closest neighbour, and distance to
    // map distance junction_box pair

    let mut distance_list: Vec<(i64, &Point3, &Point3)> = Vec::new();

    for junction_box in &junction_boxes {
        let distance_specs: Vec<(i64, &Point3, &Point3)> = junction_boxes
            .iter()
            .map(|other_box| (junction_box.euclidean(other_box), junction_box, other_box))
            .filter(|(distance, _a, _b)| *distance != 0)
            .collect();
        distance_list.extend_from_slice(&distance_specs);
    }

    // Sort by the key (the distance)
    distance_list.sort_by_key(|(dist, _, _)| *dist);

    let mut circuits: HashMap<Point3, u64> = HashMap::new();
    let mut circuit_id: u64 = 0;
    let mut connection_set: HashSet<(Point3, Point3)> = HashSet::new();

    // connect first
    let (_distance, point_a, point_b) = distance_list[0];
    println!(
        "adding points {:?} and {:?} to circuit id: {}",
        point_a, point_b, circuit_id
    );
    connection_set.insert((*point_a, *point_b));

    circuits.insert(*point_a, 0);
    circuits.insert(*point_b, 0);

    let mut connections: u64 = 1;
    let mut distance_iter = distance_list.into_iter().skip(1);
    // Print
    while connections < 10 {
        let (dist, p1, p2) = distance_iter.next().unwrap();
        if connection_set.contains(&(*p2, *p1)) {
            // already parsed the reverse version
            continue;
        } else {
            println!("{dist}: {:?} ↔ {:?}", p1, p2);
            connection_set.insert((*p1, *p2));
        }
        //println!("checking junction boxes, current circuit_id = {}", circuit_id);
        if circuits.contains_key(p1) && !circuits.contains_key(p2) {
            // connect p2 to existing circuit
            // existing circuit_id
            let existing_id = circuits.get(p1).unwrap();
            println!("adding point {:?} to circuit id: {}", p2, existing_id);
            connections += 1;
            circuits.insert(*p2, *existing_id);
        } else if circuits.contains_key(p2) && !circuits.contains_key(p1) {
            // connect p1 to existing circuit
            let existing_id = circuits.get(p2).unwrap();
            println!("adding point {:?} to circuit id: {}", p1, existing_id);
            connections += 1;
            circuits.insert(*p1, *existing_id);
        } else if !circuits.contains_key(p1) && !circuits.contains_key(p2) {
            // new circuit
            circuit_id += 1;
            println!(
                "adding points {:?} and {:?} to circuit id: {}",
                p1, p2, circuit_id
            );
            connections += 1;
            circuits.insert(*p2, circuit_id);
            circuits.insert(*p1, circuit_id);
        } else if circuits.contains_key(p2) && circuits.contains_key(p1) {
            if circuits.get(p1) != circuits.get(p2) {
                println!("merging two circuits");
                connections += 1;
                // for circuit items with id equal to p1, make it equal to p2.
                let merge_list = circuits.clone();
                for (key, value) in merge_list.iter() {
                    if value == merge_list.get(p1).unwrap() {
                        circuits.insert(*key, *merge_list.get(p2).unwrap());
                    }
                }
            } else {
                // both in same circuit, nothing to do here
                connections += 1;
                println!("nothing to do here")
            }
        }
        println!("{:?}", count_occurrences(circuits.values()));
    }

    println!("{:?}", count_occurrences(circuits.values()));
    count_occurrences(circuits.values())[0..=2].iter().product()
}

fn solve2(input: &str) -> usize {
    let junction_boxes: Vec<Point3> = parse_points(input).unwrap();
    // for each point find its closest neighbour, and distance to
    // map distance junction_box pair

    let mut distance_list: Vec<(i64, &Point3, &Point3)> = Vec::new();

    for junction_box in &junction_boxes {
        let distance_specs: Vec<(i64, &Point3, &Point3)> = junction_boxes
            .iter()
            .map(|other_box| (junction_box.euclidean(other_box), junction_box, other_box))
            .filter(|(distance, _a, _b)| *distance != 0)
            .collect();
        distance_list.extend_from_slice(&distance_specs);
    }

    // Sort by the key (the distance)
    distance_list.sort_by_key(|(dist, _, _)| *dist);

    let mut circuits: HashMap<Point3, u64> = HashMap::new();
    let mut circuit_id: u64 = 0;
    let mut connection_set: HashSet<(Point3, Point3)> = HashSet::new();

    // connect first
    let (_distance, point_a, point_b) = distance_list[0];
    println!(
        "adding points {:?} and {:?} to circuit id: {}",
        point_a, point_b, circuit_id
    );
    connection_set.insert((*point_a, *point_b));

    circuits.insert(*point_a, 0);
    circuits.insert(*point_b, 0);

    let mut distance_iter = distance_list.into_iter().skip(1);
    // Print
    let mut answer = 0;

    while count_occurrences(circuits.values())
        .into_iter()
        .max()
        .unwrap()
        < junction_boxes.len()
    {
        let (dist, p1, p2) = distance_iter.next().unwrap();
        if connection_set.contains(&(*p2, *p1)) {
            // already parsed the reverse version
            continue;
        } else {
            println!("{dist}: {:?} ↔ {:?}", p1, p2);
            connection_set.insert((*p1, *p2));
        }
        //println!("checking junction boxes, current circuit_id = {}", circuit_id);
        if circuits.contains_key(p1) && !circuits.contains_key(p2) {
            // connect p2 to existing circuit
            // existing circuit_id
            let existing_id = circuits.get(p1).unwrap();
            println!("adding point {:?} to circuit id: {}", p2, existing_id);
            circuits.insert(*p2, *existing_id);
        } else if circuits.contains_key(p2) && !circuits.contains_key(p1) {
            // connect p1 to existing circuit
            let existing_id = circuits.get(p2).unwrap();
            println!("adding point {:?} to circuit id: {}", p1, existing_id);
            circuits.insert(*p1, *existing_id);
        } else if !circuits.contains_key(p1) && !circuits.contains_key(p2) {
            // new circuit
            circuit_id += 1;
            println!(
                "adding points {:?} and {:?} to circuit id: {}",
                p1, p2, circuit_id
            );
            circuits.insert(*p2, circuit_id);
            circuits.insert(*p1, circuit_id);
        } else if circuits.contains_key(p2) && circuits.contains_key(p1) {
            if circuits.get(p1) != circuits.get(p2) {
                println!("merging two circuits");
                // for circuit items with id equal to p1, make it equal to p2.
                let merge_list = circuits.clone();
                for (key, value) in merge_list.iter() {
                    if value == merge_list.get(p1).unwrap() {
                        circuits.insert(*key, *merge_list.get(p2).unwrap());
                    }
                }
            } else {
                // both in same circuit, nothing to do here
                println!("nothing to do here")
            }
        }
        println!("{:?}", count_occurrences(circuits.values()));
        if count_occurrences(circuits.values())
            .into_iter()
            .max()
            .unwrap()
            == junction_boxes.len()
        {
            println!(
                "chain complete as a resulting of merging: {:?} and {:?} answer : {} ",
                p1,
                p2,
                p1.x * p2.x
            );
            answer = p1.x * p2.x;
            break;
        }
    }
    answer as usize
}

fn count_occurrences(list: Values<Point3, u64>) -> Vec<usize> {
    let mut map = HashMap::new();
    for &v in list {
        *map.entry(v).or_insert(0) += 1;
    }
    let mut values: Vec<usize> = map.into_values().collect();
    values.sort_by(|a, b| b.cmp(a));
    values
}

fn main() {
    let input = get_input("day8-input.txt");

    println!("{}", solve(&input));
    println!("{}", solve2(&input));
}
