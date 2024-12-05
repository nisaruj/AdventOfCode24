use regex::Regex;
use std::fs;

fn solve(instruction: &str, enable_do_dont: bool) -> u64 {
    let re_operands = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_operator = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let ops: Vec<&str> = re_operator
        .find_iter(instruction)
        .map(|m| m.as_str())
        .collect();

    let mut mulsum = 0;
    let mut enabled = true;
    for op in ops {
        match op {
            "don't()" => {
                enabled = false;
            }
            "do()" => {
                enabled = true;
            }
            _ => {
                let cap = re_operands.captures_iter(op).next().unwrap();
                let (_, [a, b]) = cap.extract();
                if !enable_do_dont || enabled {
                    mulsum += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
                }
            }
        }
    }

    mulsum
}

fn main() {
    let input_file = "input.txt";
    let instruction: &str = &fs::read_to_string(input_file).unwrap();

    println!("Sum: {}", solve(instruction, false));
    println!("Sum: {}", solve(instruction, true));
}
