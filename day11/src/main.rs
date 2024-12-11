use std::collections::HashMap;
use std::fs;

fn stones_after_n_blinks(
    max_depth: u64,
    stone: u64,
    current_depth: u64,
    mem: &mut HashMap<(u64, u64), u64>, // DP Memoization (num, depth) -> # Stones
) -> u64 {
    if max_depth == current_depth {
        // Base Case
        return 1;
    }

    let result;

    // If key is found, then no need to compute again
    if mem.contains_key(&(stone, current_depth)) {
        return *mem.get(&(stone, current_depth)).unwrap();
    }

    // Key not found, compute the number of stones
    if stone == 0 {
        result = stones_after_n_blinks(max_depth, 1, current_depth + 1, mem);
    } else {
        let stone_str = stone.to_string();
        let len = stone_str.len();

        if len % 2 == 0 {
            result = stones_after_n_blinks(
                max_depth,
                stone_str[..(len / 2)].parse::<u64>().unwrap(),
                current_depth + 1,
                mem,
            ) + stones_after_n_blinks(
                max_depth,
                stone_str[(len / 2)..].parse::<u64>().unwrap(),
                current_depth + 1,
                mem,
            );
        } else {
            result = stones_after_n_blinks(max_depth, stone * 2024, current_depth + 1, mem);
        }
    }

    // Save the result for further use
    mem.insert((stone, current_depth), result);

    result
}

fn solve(stones: &Vec<u64>, max_depth: u64) -> u64 {
    let mut mem: HashMap<(u64, u64), u64> = HashMap::new();
    let mut total_stones: u64 = 0;

    for &stone in stones {
        total_stones += stones_after_n_blinks(max_depth, stone, 0, &mut mem);
    }

    total_stones
}

fn main() {
    let input_file = "input.txt";
    let stones: Vec<u64> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|e| e.parse::<u64>().unwrap())
        .collect();

    println!("Stone Count, Depth 25: {}", solve(&stones, 25));
    println!("Stone Count, Depth 75: {}", solve(&stones, 75));
}
