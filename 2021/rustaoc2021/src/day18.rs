// [[[[1,1],[2,2]],[3,3]],[4,4]]
// should result in
// [1,1]
// [2,2]
// [3,3]
// [4,4]

pub fn part_1() {
    // take some kind of weirdly nested tuple
    let test = ((((1,1),(2,2)),(3,3)),(4,4));

}

fn parse_snail_fish(mut number : &str) {
    number.strip_prefix("[");
    number.strip_suffix("]");

    let mut number : Vec<char> = number.chars().collect();
    let mut number = &number[..];

    if number[0] == '[' {
        println!("going deeper");
    }
    number = &number[1..];
     println!("{:?}", number)
}

#[cfg(test)]
mod tests {
    use crate::day18::{parse_snail_fish, part_1};

    #[test]
    fn test_part_1() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]";

        parse_snail_fish("[[3,2],1]")
    }
}

