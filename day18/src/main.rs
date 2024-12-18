use std::collections::VecDeque;
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const MAP_SIZE: (usize, usize) = (71, 71);

fn simulate(coords: &Vec<(usize, usize)>, size: (usize, usize)) -> Option<usize> {
    let (height, width) = size;
    let mut map = vec![vec!['.'; width]; height];
    for (i, j) in coords {
        map[*i][*j] = '#';
    }

    // (i, j, distance)
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, 0));
    while !queue.is_empty() {
        let (i, j, distance) = queue.pop_front().unwrap();

        if i == height - 1 && j == width - 1 {
            return Some(distance);
        }

        if map[i][j] == '#' {
            continue;
        }

        map[i][j] = '#'; // Mark as visited

        for dir in DIRECTIONS {
            let new_i = i as i32 + dir.0;
            let new_j = j as i32 + dir.1;

            if new_i >= 0
                && new_j >= 0
                && (new_i as usize) < height
                && (new_j as usize) < width
                && map[new_i as usize][new_j as usize] != '#'
            {
                queue.push_back((new_i as usize, new_j as usize, distance + 1));
            }
        }
    }

    None
}

fn main() {
    let input_file = "input.txt";
    let coords: Vec<(usize, usize)> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            (
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<usize>().unwrap(),
            )
        })
        .collect();

    println!(
        "Shortest Distance for the first kilobyte {}",
        simulate(&coords[..1024].to_vec(), MAP_SIZE).unwrap()
    );

    for i in 1024..coords.len() {
        let result = simulate(&coords[..i].to_vec(), MAP_SIZE);
        if result.is_none() {
            println!(
                "First Coord that blocks the path: coord[{}] = {},{}",
                i - 1,
                coords[i - 1].0,
                coords[i - 1].1,
            );
            break;
        }
    }
}
