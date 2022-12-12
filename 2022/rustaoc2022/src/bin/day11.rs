use crate::Operator::Add;

const EXAMPLE: &str = include_str!(r"../../resources/day11-example.txt");
const INPUT: &str = include_str!(r"../../resources/day11-input.txt");

enum Operator {
    Add(u64),
    Multiply(u64),
    PowerTwo(),
}

struct Monkey {
    divisor: u64,
    worry_operator: Operator,
    current_objects: Vec<u64>,
    condition_true_monkey: usize,
    condition_false_monkey: usize,
    inspections: u64,
}

fn part1(input: &str) -> u64 {
    let monkey_specs: Vec<&str> = input.trim_end().split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = monkey_specs
        .iter()
        .map(|monkey| {
            let mut monkey_spec = monkey.split("\n");
            let _monkey_name = monkey_spec.next();
            let starting_items = monkey_spec
                .next()
                .unwrap()
                .split(":")
                .skip(1)
                .next()
                .unwrap();
            let starting_items: Vec<u64> = starting_items
                .split(",")
                .map(|item| item.trim().parse::<u64>().unwrap())
                .collect();
            let operation = &monkey_spec.next().unwrap()[23..];

            let operator = match operation {
                "* old" => Operator::PowerTwo(),
                _ if operation.chars().nth(0).unwrap() == '+' => {
                    Add(operation[2..].parse::<u64>().unwrap())
                }
                _ if operation.chars().nth(0).unwrap() == '*' => {
                    Operator::Multiply(operation[2..].parse::<u64>().unwrap())
                }
                _ => panic!("error"),
            };
            let divisor = monkey_spec.next().unwrap()[21..].parse::<u64>().unwrap();
            let monkey_true = monkey_spec.next().unwrap()[29..].parse::<usize>().unwrap();
            let monkey_false = monkey_spec.next().unwrap()[30..].parse::<usize>().unwrap();
            Monkey {
                divisor,
                worry_operator: operator,
                current_objects: starting_items,
                condition_true_monkey: monkey_true,
                condition_false_monkey: monkey_false,
                inspections: 0,
            }
        })
        .collect();

    // for (index, monkey) in monkeys.iter().enumerate() {
    //     println!("Monkey {}, items {:?}", index, monkey.current_objects)
    // }

    for _round in 1..21 {
        for index in 0..monkeys.len() {
            //let items = &monkey.current_objects;
            let mut moved_items: Vec<(usize, u64)> = Vec::new();
            {
                let mut monkey = monkeys.get_mut(index).unwrap();
                let items = &monkey.current_objects;

                for item in items {
                    monkey.inspections += 1;
                    let new_worry = match monkey.worry_operator {
                        Add(value) => item + value,
                        Operator::Multiply(value) => item * value,
                        Operator::PowerTwo() => item * item,
                    } / 3;
                    if new_worry % monkey.divisor == 0 {
                        //monkeys.get_mut(monkey.condition_true_monkey).unwrap().current_objects.push(new_worry);
                        moved_items.push((monkey.condition_true_monkey, new_worry))
                    } else {
                        //monkeys.get_mut(monkey.condition_false_monkey).unwrap().current_objects.push(new_worry);
                        moved_items.push((monkey.condition_false_monkey, new_worry))
                    }
                }
                monkey.current_objects = Vec::new();
            }

            for (monkey_index, item) in moved_items {
                monkeys
                    .get_mut(monkey_index)
                    .unwrap()
                    .current_objects
                    .push(item);
            }
        }

        // println!("Round {}", round);
        // for (index, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}, items {:?}", index, monkey.current_objects)
        // }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<u64>>();
    monkey_business.sort();
    monkey_business.iter().rev().take(2).product()
}

fn part2(input: &str) -> u64 {
    let monkey_specs: Vec<&str> = input.trim_end().split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = monkey_specs
        .iter()
        .map(|monkey| {
            let mut monkey_spec = monkey.split("\n");
            let _monkey_name = monkey_spec.next();
            let starting_items = monkey_spec
                .next()
                .unwrap()
                .split(":")
                .skip(1)
                .next()
                .unwrap();
            let starting_items: Vec<u64> = starting_items
                .split(",")
                .map(|item| item.trim().parse::<u64>().unwrap())
                .collect();
            let operation = &monkey_spec.next().unwrap()[23..];

            let operator = match operation {
                "* old" => Operator::PowerTwo(),
                _ if operation.chars().nth(0).unwrap() == '+' => {
                    Add(operation[2..].parse::<u64>().unwrap())
                }
                _ if operation.chars().nth(0).unwrap() == '*' => {
                    Operator::Multiply(operation[2..].parse::<u64>().unwrap())
                }
                _ => panic!("error"),
            };
            let divisor = monkey_spec.next().unwrap()[21..].parse::<u64>().unwrap();
            let monkey_true = monkey_spec.next().unwrap()[29..].parse::<usize>().unwrap();
            let monkey_false = monkey_spec.next().unwrap()[30..].parse::<usize>().unwrap();
            Monkey {
                divisor,
                worry_operator: operator,
                current_objects: starting_items,
                condition_true_monkey: monkey_true,
                condition_false_monkey: monkey_false,
                inspections: 0,
            }
        })
        .collect();

    // common factor
    let factor: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();
    for _round in 1..10001 {
        for index in 0..monkeys.len() {
            //let items = &monkey.current_objects;
            let mut moved_items: Vec<(usize, u64)> = Vec::new();
            {
                let mut monkey = monkeys.get_mut(index).unwrap();
                let items = &monkey.current_objects;

                for item in items {
                    monkey.inspections += 1;
                    let new_worry = match monkey.worry_operator {
                        Add(value) => item + value,
                        Operator::Multiply(value) => item * value,
                        Operator::PowerTwo() => item * item,
                    } % factor;
                    if new_worry % monkey.divisor == 0 {
                        //monkeys.get_mut(monkey.condition_true_monkey).unwrap().current_objects.push(new_worry);
                        moved_items.push((monkey.condition_true_monkey, new_worry))
                    } else {
                        //monkeys.get_mut(monkey.condition_false_monkey).unwrap().current_objects.push(new_worry);
                        moved_items.push((monkey.condition_false_monkey, new_worry))
                    }
                }
                monkey.current_objects = Vec::new();
            }

            for (monkey_index, item) in moved_items {
                monkeys
                    .get_mut(monkey_index)
                    .unwrap()
                    .current_objects
                    .push(item);
            }
        }

        // println!("Round {}", round);
        // for (index, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}, items {:?}", index, monkey.current_objects)
        // }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<u64>>();
    println!("{:?}", monkey_business);
    monkey_business.sort();
    monkey_business.iter().rev().take(2).product()
}

fn main() {
    rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, EXAMPLE, INPUT};

    #[test]
    fn test_example() {
        assert_eq!(10605, part1(EXAMPLE));
        assert_eq!(2713310158, part2(EXAMPLE));
    }

    #[test]
    fn test_input() {
        assert_eq!(102399, part1(INPUT));
        assert_eq!(23641658401, part2(INPUT));
    }
}
