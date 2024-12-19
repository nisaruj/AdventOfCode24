use std::fs;

fn solve(patterns: Vec<&str>, query: String) -> u64 {
    let mut dp: Vec<u64> = vec![0; query.len() + 1];

    dp[0] = 1;
    for i in 1..query.len() + 1 {
        for pat in patterns.iter() {
            if i >= pat.len() && query[(i - pat.len())..i].starts_with(pat) {
                dp[i] += dp[i - pat.len()];
            }
        }
    }

    dp[query.len()] as u64
}

fn main() {
    let input_file = "input.txt";

    let fp = fs::read_to_string(input_file).unwrap();
    let mut line_iter = fp.lines();

    let patterns: Vec<&str> = line_iter.next().unwrap().split(", ").collect();
    let mut possible_query = 0;
    let mut sum_possible_ways = 0;

    line_iter.next();
    let mut query = line_iter.next();

    while query.is_some() {
        let possible_ways = solve(patterns.clone(), query.unwrap().to_string());

        if possible_ways > 0 {
            possible_query += 1;
        }

        sum_possible_ways += possible_ways;

        query = line_iter.next();
    }

    println!("Possible Queries {}", possible_query);
    println!("Sum Possible Ways {}", sum_possible_ways);
}
