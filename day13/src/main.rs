use regex::Regex;
use std::fs;

const BUTTON_A_TOKENS: i64 = 3;
const BUTTON_B_TOKENS: i64 = 1;
const PRIZE_OFFSET: i64 = 10000000000000;

#[derive(Debug)]
struct Query {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn solve(q: &Query, prize_offset: i64) -> Option<i64> {
    // Add prize offset for part 2
    let query = Query {
        button_a: q.button_a,
        button_b: q.button_b,
        prize: (q.prize.0 + prize_offset, q.prize.1 + prize_offset),
    };

    // Basically solving 2 algebraic equations w/ 2 variables (Cramer's rules)
    let coef = query.button_a.0 * query.button_b.1 - query.button_a.1 * query.button_b.0;
    let prize_const = query.prize.0 * query.button_b.1 - query.prize.1 * query.button_b.0;

    if prize_const % coef == 0 {
        let a_presses = prize_const / coef;
        if (query.prize.0 - query.button_a.0 * a_presses) % query.button_b.0 == 0 {
            let b_presses = (query.prize.0 - query.button_a.0 * a_presses) / query.button_b.0;

            return Some(BUTTON_A_TOKENS * a_presses + BUTTON_B_TOKENS * b_presses);
        } else {
            // Not integer
            return None;
        }
    } else {
        // Not integer
        return None;
    }
}

fn main() {
    let input_file = "input.txt";
    let input_str = fs::read_to_string(input_file).unwrap();

    let re = Regex::new(r".*\+(\d+),.*\+(\d+)\n.*\+(\d+),.*\+(\d+)\n.*\=(\d+),.*\=(\d+)").unwrap();
    let queries: Vec<Query> = re
        .captures_iter(input_str.as_str())
        .map(|caps| {
            let (_, extracted) = caps.extract();
            let [a_x, a_y, b_x, b_y, p_x, p_y] = extracted.map(|s| s.parse::<i64>().unwrap());

            Query {
                button_a: (a_x, a_y),
                button_b: (b_x, b_y),
                prize: (p_x, p_y),
            }
        })
        .collect();

    let mut sum: (i64, i64) = (0, 0);
    for query in queries {
        // Part 1
        let token = solve(&query, 0);
        if token.is_some() {
            sum.0 += token.unwrap();
        }

        // Part 2
        let token = solve(&query, PRIZE_OFFSET);
        if token.is_some() {
            sum.1 += token.unwrap();
        }
    }

    println!("Min Token {}, {}", sum.0, sum.1);
}
