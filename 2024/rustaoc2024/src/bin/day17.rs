use std::fs;
use rustaoc2024::get_input;


fn part_b(input: &str) -> i64 {

    let lines: Vec<&str> = input.lines().collect();
    // Parse registers
    let reg_a = lines[0][12..].parse::<i64>().unwrap();
    let reg_b = lines[1][12..].parse::<i64>().unwrap();
    let reg_c = lines[2][12..].parse::<i64>().unwrap();
    // Parse program
    let program: Vec<i64> = lines[4][9..]
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let mut sum = 0;
    for (index, bit) in program.iter().enumerate() {
        let bit_value = bit << (index)* 3;
        sum += bit_value;
        // println!("bit value = {} achieved by shifting {} by {} bits", bit_value, bit, index * 3);
    }
    // println!("({})", sum);
    println!("searching for input for program {:?}", program);
    if let Some(value) = find_lowest_a(0, reg_b, reg_c, &*program, 1) {
        value
    } else { 0 }
}

fn find_lowest_a(a: i64, b: i64, c: i64, program: &[i64], depth : usize) -> Option<i64> {
    // shift by 3 for every recursion
    if depth > program.len() {
        return None;
    }
    let mut a = a << 3;
        for check in 0..64 {
            let output = simulate(a+check, b, c, program);
            let correct = output.iter().rev().enumerate().all( | (index, value) | program[program.len() - index - 1] == *value);
            if correct && output.len() == program.len() {
                println!("exact match found for a = {} " , a+check );
                println!("Program: {:?}", program.iter().rev().collect::<Vec<&i64>>());
                println!("Output : {:?}", output.iter().rev().collect::<Vec<&i64>>());
                return Some(a+check);
            }
            if correct {
                println!("Correct value {} current depth {}", a+check, depth);
                println!("Program: {:?}", program.iter().rev().collect::<Vec<&i64>>());
                println!("Output : {:?}", output.iter().rev().collect::<Vec<&i64>>());
                // println!("");
                if let Some(result) = find_lowest_a(a+check, b, c, program, depth + 1) {
                    return Some(result);
                } else {
                    continue;
                }

                // println!("Wrong value {}, depth {}", a + check, depth);
                // println!("Program: {:?}", program);
                // println!("Output : {:?}", output);
            }

    }
    None
}
fn part_a(input: &str) -> Vec<i64> {
    let lines: Vec<&str> = input.lines().collect();
    // Parse registers
    let reg_a = lines[0][12..].parse::<i64>().unwrap();
    let reg_b = lines[1][12..].parse::<i64>().unwrap();
    let reg_c = lines[2][12..].parse::<i64>().unwrap();
    // Parse program
    let program: Vec<i64> = lines[4][9..]
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    // Simulate program
    simulate(reg_a, reg_b, reg_c, &program)
}

fn simulate(mut a: i64, mut b: i64, mut c: i64, program: &[i64]) -> Vec<i64> {
    let mut ip: usize = 0;
    let mut output = Vec::new();

    while ip < program.len() {
        //println!("{},{},{}", a, b, c);
        let opcode = program[ip];
        let operand = program[ip + 1];
        match opcode {
            0 => { // adv
                let divisor = 1 << combo_value(operand, a, b, c);
                a /= divisor;
                ip += 2;
            }
            1 => { // bxl
                b ^= operand as i64;
                ip += 2;
            }
            2 => { // bst
                b = combo_value(operand, a, b, c) % 8;
                ip += 2;
            }
            3 => { // jnz
                if a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            4 => { // bxc
                b ^= c;
                ip += 2;
            }
            5 => { // out
                output.push(combo_value(operand, a, b, c) % 8);
                ip += 2;
            }
            6 => { // bdv
                let divisor = 1 << combo_value(operand, a, b, c);
                b = a / divisor;
                ip += 2;
            }
            7 => { // cdv
                let divisor = 1 << combo_value(operand, a, b, c);
                c = a / divisor;
                ip += 2;
            }
            _ => panic!("Invalid opcode"),
        }
    }
    // println!("output={:?}", output);
    output
}

fn combo_value(operand: i64, a: i64, b: i64, c: i64) -> i64 {
    match operand {
        0..=3 => operand as i64,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid combo operand"),
    }
}

fn main() {
    let input = get_input("day17-input.txt");
    let result = part_a(&input);
    println!("{:?}", result);
    println!("{}", part_b(&input));

}