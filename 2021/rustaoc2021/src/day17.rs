// target area: x=241..273, y=-97..-63
// from start of 0,0 how do you reach that spot?
// what initial x,y velocity do you need?

// initial_x must be equal to the number of steps needed for y times distance required
// given we want highest position, can we reach -63 (highest value)

// x distance each step reduces by 1. We have a 241 - 273 window = 32
// these problems are correlated in the number of steps required. It seems x puts a bound on x

use std::ops::Range;

fn calculate_velocity_required(range_x_min : i64, range_x_max: i64, range_y_min : i64, range_y_max: i64) -> Vec<(i64, i64 , i64)> {
    let mut results = Vec::new();

    for x_test_initial in 0..1000 {
        'inner_initial_spec: for y_test_initial in 0..1000 {
            // run x iterations
            let mut x_current = x_test_initial.clone()-500;
            let mut y_current = y_test_initial.clone()-500;
            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut y_max = i64::MIN;

            for n in 0..500 {
                x_pos += x_current;
                y_pos += y_current;
                if y_pos > y_max {
                    y_max = y_pos;
                }

                if x_current > 0 {
                    x_current -= 1
                } else if x_current < 0 {
                    x_current += 1
                }

                y_current -= 1;

                if x_pos >= range_x_min && x_pos <= range_x_max && y_pos >= range_y_min && y_pos <= range_y_max {
                    println!("current value in target area: {}, {} after {} steps, using intial values {},{}. Highest y: {}", x_current, y_current, n, x_test_initial, y_test_initial, y_max);
                    results.push((x_test_initial, y_test_initial, y_max));
                    continue 'inner_initial_spec
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::day17::calculate_velocity_required;

    #[test]
    fn test_calc_intersect() {
        // target area: x=241..273, y=-97..-63
        let results = calculate_velocity_required(241,273, -97, -63);
        println!("{:?}",results);
        let answer = results.iter().map( |(_x_i, y_i, y_max)| y_max).max().unwrap();
        // 1128 is too low
        assert_eq!(&4656, answer);
        assert_eq!(112, results.len())
    }

    #[test]
    fn test_calc_intersect_example() {
        // target area: x=241..273, y=-97..-63
        let results = calculate_velocity_required(20,30, -10, -5);
        println!("{:?}",results);
        let answer = results.iter().map( |(_x_i, y_i, y_max)| y_max).max().unwrap();
        assert_eq!(&45, answer);
        assert_eq!(112, results.len())
    }

}