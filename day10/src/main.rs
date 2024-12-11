use std::collections::HashSet;
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn find_top_positions(
    map: &Vec<Vec<u8>>,
    curr_node: (usize, usize),
) -> (HashSet<(usize, usize)>, usize) {
    let (i, j) = curr_node;
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    if map[i][j] == 9 {
        return (HashSet::from_iter([curr_node]), 1);
    }

    let mut top_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut trail_count: usize = 0;

    for dir in DIRECTIONS {
        let new_i = i as i32 + dir.0;
        let new_j = j as i32 + dir.1;
        if new_i >= 0
            && new_i < height
            && new_j >= 0
            && new_j < width
            && map[i as usize][j as usize] + 1 == map[new_i as usize][new_j as usize]
        {
            let (tp, tc) = find_top_positions(map, (new_i as usize, new_j as usize));
            top_positions.extend(tp);
            trail_count += tc;
        }
    }

    (top_positions, trail_count)
}

fn calculate_score(map: &Vec<Vec<u8>>) -> (usize, usize) {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut score = 0;
    let mut rating = 0;

    for i in 0..height {
        for j in 0..width {
            if map[i as usize][j as usize] == 0 {
                let (tp, tc) = find_top_positions(&map, (i as usize, j as usize));
                score += tp.len();
                rating += tc;
            }
        }
    }

    (score, rating)
}

fn main() {
    let input_file = "input.txt";
    let map: Vec<Vec<u8>> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    println!("(Score, Rating) = {:?}", calculate_score(&map));
}
