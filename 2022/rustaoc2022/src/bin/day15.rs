use colored::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;

const EXAMPLE: &str = include_str!(r"../../resources/day15-example.txt");
const INPUT: &str = include_str!(r"../../resources/day15-input.txt");

type Location = (i64, i64);

fn part1(input: &str) -> i64 {
    let mut pairs: Vec<(Location, Location, i64)> = Vec::new();
    let mut beacons: HashSet<Location> = HashSet::new();

    for line in input.lines() {
        let (sensor, beacon) = parse_line(line);
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        pairs.push((sensor, beacon, distance));
        beacons.insert(beacon);
        println!("{:?}    {:?}    {}", sensor, beacon, distance)
    }

    // eliminate locations with same or smaller distance
    // distance is manhattan so sum x+y or less.
    let mut all_locations: HashSet<Location> = HashSet::new();
    let mut target_line = 10;
    if pairs.len() > 20 {
        target_line = 2000000;
    }

    for (sensor, beacon, distance) in pairs {
        println!(
            "now evaluating sensor {:?} with nearest beacon {:?}",
            sensor, beacon
        );
        get_blocked_locations_on_line(sensor, distance, target_line, &mut all_locations);
    }

    // filter out beacons
    let all_locations = all_locations.difference(&beacons);

    let value = all_locations.count();
    // println!("{} locations blocked on line {}", value, target_line);
    value as i64
}

fn get_blocked_locations_on_line(
    sensor: Location,
    distance: i64,
    line_y: i64,
    locations: &mut HashSet<Location>,
) {
    //println!("manhattan distance between sensor and beacon is {}", distance);
    let y_diff = (sensor.1 - line_y).abs();
    let location_offset = distance - y_diff;

    // if manhattan distance is larger than diff between line_x and sensor_x -> no matches
    if location_offset <= 0 {
        return;
    } else {
        // if distance is smaller. create all points on x, line_y with a smaller or equal manhattan distance
        // println!("with a manhattan distance of {} and ydiff off {} we can have {} offsets", max_beacon_dist, y_diff, location_offset);
        for offset in -location_offset..=location_offset {
            let position = (offset + sensor.0, line_y);
            // could eliminate those afterwards instead of every loop
            locations.insert(position);
        }
    }
}

fn get_allowed_locations_offset_1(
    sensor: Location,
    distance: i64,
    locations: &mut HashSet<Location>,
) {
    //let mut locations = HashSet::new();
    // generate the diamond

    //    0 1 2 3
    // 0    *
    // 1  * # *   // if distance is 0, x ofsset is distance +- 1
    // 2    *

    // triangle so increase one while decreasing other.
    for x_offset in 0..=distance + 1 {
        // y becomes higher
        // x_offset becomes larger
        //      *
        //     * *
        // Y_offset : start at distance + 1
        // ends at -1
        // X_offset: starts at 0 ends at distance
        let y_offset = distance + 1 - x_offset;
        locations.insert((sensor.0 - x_offset, sensor.1 + y_offset));
        // println!("offset x: {}, offset y: {}", sensor.0 + x_offset, sensor.1 + y_offset );
        // println!("offset x: {}, offset y: {}", sensor.0 - x_offset, sensor.1 + y_offset );
        // println!("offset x: {}, offset y: {}", sensor.0 + x_offset, sensor.1 - y_offset );
        // println!("offset x: {}, offset y: {}", sensor.0 - x_offset, sensor.1 - y_offset );
        locations.insert((sensor.0 + x_offset, sensor.1 + y_offset));
        locations.insert((sensor.0 - x_offset, sensor.1 - y_offset));
        locations.insert((sensor.0 + x_offset, sensor.1 - y_offset));
    }
}

// Sensor at x=2, y=18:
// closest beacon is at x=-2, y=15
fn parse_line(line: &str) -> (Location, Location) {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut capture = re.find_iter(line);
    let point_a: Location = (
        capture.next().unwrap().as_str().parse::<i64>().unwrap(),
        capture.next().unwrap().as_str().parse::<i64>().unwrap(),
    );
    let point_b: Location = (
        capture.next().unwrap().as_str().parse::<i64>().unwrap(),
        capture.next().unwrap().as_str().parse::<i64>().unwrap(),
    );
    (point_a, point_b)
}

fn part2(_input: &str) -> i64 {
    let mut pairs: Vec<(Location, Location, i64)> = Vec::new();
    let mut beacons: HashSet<Location> = HashSet::new();

    for line in _input.lines() {
        let (sensor, beacon) = parse_line(line);
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        pairs.push((sensor, beacon, distance));
        beacons.insert(beacon);
        //println!("{:?}    {:?}    {}", sensor, beacon, distance)
    }

    // eliminate locations with same or smaller distance
    // distance is manhattan so sum x+y or less.
    let mut all_locations: HashSet<Location> = HashSet::new();

    // get all of the edges now find an edge that overlaps
    // let mut locs: HashMap<Location, u64>  = HashMap::new();
    // let mut test = HashSet::new();
    for (sensor, beacon, distance) in &pairs {
        println!(
            "now evaluating sensor {:?} with nearest beacon {:?}",
            sensor, beacon
        );
        get_allowed_locations_offset_1(*sensor, *distance, &mut all_locations);
    }

    let limits = {
        if pairs.len() > 20 {
            4000000
        } else {
            10
        }
    };
    // search locations on the edge of current beacon ranges
    // for each of those locations check if they are in range of a sensor, beacon pair
    //println!("found {} possible locations", all_locations.len());
    let mut final_location: Location = (0, 0);
    'location: for location in all_locations {
        if (location.0 < 0 || location.0 > limits || location.1 < 0 || location.1 > limits) {
            continue 'location;
        }

        let mut found = true;

        'sensor: for (sensor, _, distance) in &pairs {
            // if distance smaller than range -> not allowed break this location
            if ((location.0 - sensor.0).abs() + (location.1 - sensor.1).abs()) <= *distance {
                //println!("evaluating {:?} but ignoring because distance smaller than {} to sensor {:?}", location, distance, sensor);
                found = false;
                break 'sensor;
            }
        }

        if found {
            println!("allowed position at location {:?}", location);
            final_location = location;
            break 'location;
        }
    }
    final_location.0 * 4000000 + final_location.1
}

fn main() {
    // clear terminal
    //part2(EXAMPLE);
    rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{parse_line, part1, part2, EXAMPLE, INPUT};

    #[test]
    fn test_parse_line() {
        let line = "Sensor at x=2, y=18 : closest beacon is at x=-2, y=15";
        let (point_a, point_b) = parse_line(line);
    }

    #[test]
    #[ignore]
    fn test_example() {
        assert_eq!(24, part1(EXAMPLE));
        assert_eq!(93, part2(EXAMPLE));
    }

    #[test]
    #[ignore]
    fn test_input() {
        assert_eq!(1001, part1(INPUT));
        assert_eq!(27976, part2(INPUT));
    }
}
