use std::collections::HashMap;
use std::fs;

fn solve_part1(first_arr: &Vec<u64>, second_arr: &Vec<u64>) -> u64 {
    let mut a = first_arr.clone();
    let mut b = second_arr.clone();
    a.sort();
    b.sort();

    let mut sum_diff = 0;
    for (index, _) in a.iter().enumerate() {
        if a[index] > b[index] {
            sum_diff += a[index] - b[index];
        } else {
            sum_diff += b[index] - a[index];
        }
    }

    sum_diff
}

fn solve_part2(first_arr: &Vec<u64>, second_arr: &Vec<u64>) -> u64 {
    let mut b_map: HashMap<u64, u64> = HashMap::new();

    for &e in second_arr {
        let count = b_map.entry(e).or_insert(0);
        *count += 1;
    }

    let mut sim_score = 0;

    for &e in first_arr {
        sim_score += e * (b_map.get(&e).unwrap_or(&0));
    }

    sim_score
}

fn main() {
    let filename = "input.txt";

    let mut first_arr: Vec<u64> = Vec::new();
    let mut second_arr: Vec<u64> = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        let sline: Vec<String> = line.split("   ").map(String::from).collect();

        first_arr.push(sline[0].parse::<u64>().unwrap());
        second_arr.push(sline[1].parse::<u64>().unwrap());
    }

    println!("{}", solve_part1(&first_arr, &second_arr));
    println!("{}", solve_part2(&first_arr, &second_arr));
}
