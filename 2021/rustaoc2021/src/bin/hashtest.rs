use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    // combine two hash sets into a new owned hashset
    let hash1: HashSet<u64> = HashSet::from_iter(0..3);
    let hash2: HashSet<u64> = HashSet::from_iter(3..5);
    let hash3: HashSet<u64> = hash1.union(&hash2).copied().collect();
    println!("{:?}", hash3);

    // sort a string
    let something =  "akfc".to_string();
    let mut sorted: Vec<String> = something.chars().map(|x| x.to_string()).collect();
    sorted.sort();
    let sorted = sorted.concat();
    println!("{:?}", sorted);

}
