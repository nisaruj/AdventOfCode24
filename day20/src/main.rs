use std::collections::{HashMap, VecDeque};
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

const PART1_LIMIT: usize = 2;
const PART2_LIMIT: usize = 20;
const PICOSEC_THRESHOLD: usize = 100;

fn bfs(map: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let height = map.len();
    let width = map[0].len();
    let mut visited = vec![vec![false; width]; height];

    // (i, j, steps)
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((start.0, start.1, 0));
    visited[start.0][start.1] = true;

    while !queue.is_empty() {
        let (i, j, steps) = queue.pop_front().unwrap();

        if i == end.0 && j == end.1 {
            return Some(steps);
        }

        for (di, dj) in DIRECTIONS {
            let new_i = i as i32 + di;
            let new_j = j as i32 + dj;
            if new_i >= 0 && new_i < height as i32 && new_j >= 0 && new_j < width as i32 {
                let new_i = new_i as usize;
                let new_j = new_j as usize;

                if !visited[new_i][new_j] && map[new_i][new_j] != b'#' {
                    visited[new_i][new_j] = true;
                    queue.push_back((new_i, new_j, steps + 1));
                }
            }
        }
    }

    None
}

// Return the number of steps to reach end_pos from start_pos if we can cheat
fn cheating_distance(
    map: &Vec<Vec<u8>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    limit: usize,
) -> Option<usize> {
    if map[start_pos.0][start_pos.1] == b'#' || map[end_pos.0][end_pos.1] == b'#' {
        return None;
    }

    let distance = ((start_pos.0 as i32 - end_pos.0 as i32).abs()
        + (start_pos.1 as i32 - end_pos.1 as i32).abs()) as usize;

    if distance > 1 && distance <= limit {
        return Some(distance);
    }

    None
}

fn solve(map: &Vec<Vec<u8>>, saving_threshold: usize, cheat_distance_limit: usize) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut cheat_count = 0;

    // steps => [(i, j)]
    let mut cheatables: HashMap<usize, Vec<(usize, usize, usize, usize)>> = HashMap::new();

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for (i, line) in map.iter().enumerate() {
        if line.contains(&b'S') {
            start = (i, line.iter().position(|&x| x == b'S').unwrap());
        }
        if line.contains(&b'E') {
            end = (i, line.iter().position(|&x| x == b'E').unwrap());
        }
    }

    let normal_steps = bfs(&map, start, end).unwrap();

    // Precompute distance from start to each cell
    let mut distance_from_start: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    for i in 0..height {
        for j in 0..width {
            distance_from_start[i][j] = bfs(&map, start, (i, j));
        }
    }

    // Precompute distance from each cell to end
    let mut distance_to_end: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    for i in 0..height {
        for j in 0..width {
            distance_to_end[i][j] = bfs(&map, (i, j), end);
        }
    }

    // For a pair of coordinates, check if it is cheatable
    for i1 in 0..height {
        for j1 in 0..width {
            for i2 in 0..height {
                for j2 in 0..width {
                    if (i1, j1) != (i2, j2) {
                        let cheatable =
                            cheating_distance(&map, (i1, j1), (i2, j2), cheat_distance_limit);

                        if cheatable.is_some() {
                            let before_enter_cheat = distance_from_start[i1][j1].unwrap();
                            let cheat_steps = cheatable.unwrap();
                            let after_exit_cheat = distance_to_end[i2][j2].unwrap();

                            let steps = before_enter_cheat + cheat_steps + after_exit_cheat;

                            if normal_steps > steps && normal_steps - steps >= saving_threshold {
                                let saved = normal_steps - steps;
                                if cheatables.contains_key(&saved) {
                                    cheatables.get_mut(&saved).unwrap().push((i1, j1, i2, j2));
                                } else {
                                    cheatables.insert(saved, vec![(i1, j1, i2, j2)]);
                                }
                                cheat_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    cheat_count
}

fn main() {
    let input_file = "input.txt";

    let map: Vec<Vec<u8>> = fs::read_to_string(input_file)
        .expect("Unable to read file")
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    println!("Part 1 {}", solve(&map, PICOSEC_THRESHOLD, PART1_LIMIT));
    println!("Part 2 {}", solve(&map, PICOSEC_THRESHOLD, PART2_LIMIT));
}
