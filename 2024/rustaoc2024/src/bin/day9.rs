use std::fs;

fn part_a(input: &str) -> usize {
    let mut human_readable_map = parse_disk_map(input);
    sort_disk_map(&mut human_readable_map);
    calculate_checksum(&human_readable_map)
}

fn part_b(input: &str) -> usize {
    let mut human_readable_map = parse_disk_map(input);
    sort_disk_map_b(&mut human_readable_map);
    calculate_checksum(&human_readable_map)
}

fn main() {
    let disk_map = "2333133121414131402";
    let input = fs::read_to_string(r"2024/rustaoc2024/resources/day9-input.txt").unwrap();
    let _input = input.trim();
    println!("part a: {}", part_a(disk_map));
    println!("part b: {}", part_b(disk_map));
}

fn sort_disk_map(disk_map: &mut [Option<usize>]) {
    let disk_size = disk_map.len();
    for i in 0..disk_size {
        if disk_map[i].is_none() {
            let (offset, file_id) = disk_map
                .iter()
                .rev()
                .enumerate()
                .find(|(_y, x)| x.is_some())
                .unwrap();
            if disk_size - offset - 1 <= i {
                // done compacting
                break;
            }
            disk_map[i] = Some(file_id.unwrap());
            disk_map[disk_size - offset - 1] = None;
        }
    }
}

fn find_free_space(disk_map: &[Option<usize>], space: usize) -> Option<usize> {
    let result = disk_map
        .windows(space)
        .enumerate()
        .find(|(_, window)| window.iter().all(|x| x.is_none()));

    if let Some((index, _)) = result {
        return Some(index);
    }
    None
}

fn sort_disk_map_b(disk_map: &mut [Option<usize>]) {
    let disk_size = disk_map.len();
    // sort the other way around
    // for loop in reverse. Determine blocks of numbers. For each block after its end check if free size
    let mut current_file_id: Option<usize> = None;
    let mut current_file_blocks = 0;

    for index in 0..disk_size {
        let file_id = disk_map[disk_size - index - 1];
        if file_id != current_file_id && current_file_blocks > 0 {
            let space_index = find_free_space(disk_map, current_file_blocks);
            if let Some(space_index) = space_index {
                if space_index < disk_size - index {
                    disk_map[space_index..(space_index + current_file_blocks)]
                        .fill(current_file_id);
                    disk_map[(disk_size - index)..(disk_size - index + current_file_blocks)]
                        .fill(None);
                }
            }
            current_file_blocks = 0
        }
        if file_id.is_some() {
            current_file_blocks += 1
        }
        current_file_id = file_id;
    }
}

fn calculate_checksum(disk_map: &[Option<usize>]) -> usize {
    disk_map
        .iter()
        .enumerate()
        .fold(0, |checksum, (position, file_id)| {
            if let Some(file_id) = file_id {
                checksum + position * file_id
            } else {
                checksum
            }
        })
}

fn parse_disk_map(disk_map: &str) -> Vec<Option<usize>> {
    let mut file_id = 0;
    let mut result = Vec::new();

    for (index, c) in disk_map.chars().enumerate() {
        let size = c.to_digit(10).unwrap() as usize;

        match index % 2 {
            0 => {
                result.extend(std::iter::repeat(Some(file_id)).take(size));
                file_id += 1;
            }
            1 => {
                result.extend(std::iter::repeat(None).take(size));
            }
            _ => unreachable!(), // Should never happen
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_consecutive() {
        let result = find_free_space(&[Some(1), Some(2), Some(3)], 3);
        let result2 = find_free_space(&[None, None, None], 3);
        let result3 = find_free_space(&[Some(10), None, None, None], 3);
        assert_eq!(result, None);
        assert_eq!(result2, Some(0));
        assert_eq!(result3, Some(1));
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a("2333133121414131402"), 1928);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b("2333133121414131402"), 2858);
    }
}
