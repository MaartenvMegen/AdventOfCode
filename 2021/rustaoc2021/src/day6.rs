use std::collections::HashMap;

pub fn get_recursive_cache(seed: &mut Vec<u64>, days: u64) -> u64 {
    let mut cache: HashMap<i64, u64> = HashMap::new();

    seed.iter()
        .map(|fish| get_fishes((days - fish) as i64, &mut cache))
        .sum()
}

pub fn get_fishes(iteration: i64, cache: &mut HashMap<i64, u64>) -> u64 {
    let ages = vec![7,9];

    if iteration > 0 {
        ages.iter().map( |age| { if let Some(nr) = cache.get(&(iteration - age)) {
            *nr
        } else {
            let nr = get_fishes(iteration - age, cache);
            cache.insert(iteration-age, nr);
            nr
        }}).sum()
    } else {
        1
    }
}

fn run_seed_for_x_generations(seed: &mut Vec<u64>, iterations: i32) -> usize {
    let mut seed = seed.clone();
    for _iteration in 0..iterations {
        for i in 0..seed.len() {
            if &seed[i] == &0 {
                seed.push(8)
            }
            if &seed[i] == &0 {
                seed[i] = 6
            } else {
                seed[i] -= 1
            }
        }
    }
    seed.len()
}

pub fn get_fishes_after_x_days(seed: &mut Vec<u64>, days: usize) -> u64 {
    // setup vec of zeroes
    let mut breeders: Vec<u64> = Vec::with_capacity(9);
    for _i in 0..9 {
        breeders.push(0);
    }

    // initialize with seed
    for fish_age in seed.clone() {
        breeders[fish_age as usize] += 1
    }

    // run to completion
    for iteration in 0..days {
        let current_breeders = breeders[iteration % 9];
        // existing fish spawn a new fish at age 8 (loop size, keep existing value)
        // spawn current fish as new fish in 6 cycles
        breeders[(iteration + 7) % 9] += current_breeders;
    }
    breeders.iter().sum()
}

pub fn get_population_recursive(seed: &mut Vec<u64>, iterations: u64) -> u64 {
    let seed = seed.clone();
    let current_iteration = 0;
    let mut total_population = 0 as u64;
    for fish in seed {
        total_population += get_children(fish, current_iteration, iterations)
    }
    total_population
}

fn get_children(start_age: u64, current_iteration: u64, iterations: u64) -> u64 {
    let spawns: Vec<u64> = (current_iteration + start_age..iterations)
        .step_by(7)
        .collect();
    let mut children = 1;
    for iteration in spawns {
        children += get_children(9, iteration, iterations)
    }
    children
}

#[cfg(test)]
mod tests {
    use crate::day6::{
        get_fishes_after_x_days, get_population_recursive, get_recursive_cache,
        run_seed_for_x_generations,
    };

    #[test]
    fn test_example() {
        let mut seed = vec![3, 4, 3, 1, 2];
        //let mut seed = vec![ 3];
        assert_eq!(26, run_seed_for_x_generations(&mut seed, 18));
        assert_eq!(26, get_population_recursive(&mut seed, 18));
        assert_eq!(26, get_recursive_cache(&mut seed, 18));
        assert_eq!(26, get_fishes_after_x_days(&mut seed, 18));
        assert_eq!(26984457539, get_recursive_cache(&mut seed, 256));

        assert_eq!(26984457539, get_fishes_after_x_days(&mut seed, 256));
    }

    #[test]
    fn test_input() {
        let mut seed = vec![
            4, 1, 3, 2, 4, 3, 1, 4, 4, 1, 1, 1, 5, 2, 4, 4, 2, 1, 2, 3, 4, 1, 2, 4, 3, 4, 5, 1, 1,
            3, 1, 2, 1, 4, 1, 1, 3, 4, 1, 2, 5, 1, 4, 2, 2, 1, 1, 1, 3, 1, 5, 3, 1, 2, 1, 1, 1, 1,
            4, 1, 1, 1, 2, 2, 1, 3, 1, 3, 1, 3, 4, 5, 1, 2, 2, 1, 1, 1, 4, 1, 5, 1, 3, 1, 3, 4, 1,
            3, 2, 3, 4, 4, 4, 3, 4, 5, 1, 3, 1, 3, 5, 1, 1, 1, 1, 1, 2, 4, 1, 2, 1, 1, 1, 5, 1, 1,
            2, 1, 3, 1, 4, 2, 3, 4, 4, 3, 1, 1, 3, 5, 3, 1, 1, 5, 2, 4, 1, 1, 3, 5, 1, 4, 3, 1, 1,
            4, 2, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 4, 5, 1, 2, 5, 3, 1, 1, 3, 1, 1, 1, 1, 5, 1,
            2, 5, 1, 1, 1, 1, 1, 1, 3, 5, 1, 3, 2, 1, 1, 1, 1, 1, 1, 1, 4, 5, 1, 1, 3, 1, 5, 1, 1,
            1, 1, 3, 3, 1, 1, 1, 4, 4, 1, 1, 4, 1, 2, 1, 4, 4, 1, 1, 3, 4, 3, 5, 4, 1, 1, 4, 1, 3,
            1, 1, 5, 5, 1, 2, 1, 2, 1, 2, 3, 1, 1, 3, 1, 1, 2, 1, 1, 3, 4, 3, 1, 1, 3, 3, 5, 1, 2,
            1, 4, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 4, 5, 5, 1, 1, 1, 4, 1, 1, 1, 2, 1, 2, 1, 3,
            1, 3, 1, 1, 1, 1, 1, 1, 1, 5,
        ];
        assert_eq!(1689540415957, get_fishes_after_x_days(&mut seed, 256));
        assert_eq!(1689540415957, get_recursive_cache(&mut seed, 256))
    }
}
