use std::collections::HashSet;
use std::fs;

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

struct LoopDetector {
    collisions: HashSet<((usize, usize), (usize, usize))>,
    prev_pos: Option<(usize, usize)>,
}

impl Default for LoopDetector {
    fn default() -> Self {
        LoopDetector {
            collisions: HashSet::new(),
            prev_pos: None,
        }
    }
}

impl LoopDetector {
    fn add(&mut self, pos: (usize, usize)) -> bool {
        // Consecutive obstructions happening twice (prev_hashtag, curr_hashtag) will cause loop
        if self.prev_pos.is_some() {
            let consecutive_hashtag = (self.prev_pos.unwrap(), pos);

            if self.collisions.contains(&consecutive_hashtag) {
                // Loop
                return true;
            }
            self.collisions.insert(consecutive_hashtag);
        }

        self.prev_pos = Some(pos);
        false
    }
}

fn find_starting_point(map: &Vec<Vec<u8>>) -> Option<(i32, i32)> {
    for (i, line) in map.into_iter().enumerate() {
        let res = line.iter().position(|c| *c == b'^');

        if res.is_some() {
            return Some((i as i32, res.unwrap() as i32));
        }
    }

    return None;
}

fn walk(map: &mut Vec<Vec<u8>>) -> Option<u64> {
    let height = map.len();
    let width = map[0].len();

    let mut pos: (i32, i32) = find_starting_point(&map).unwrap();
    let mut dir: (i32, i32) = UP;
    let mut count = 1;

    let mut loop_detector: LoopDetector = Default::default();

    loop {
        if map[pos.0 as usize][pos.1 as usize] == b'.' {
            map[pos.0 as usize][pos.1 as usize] = b'X';
            count += 1;
        }

        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

        if next_pos.0 < 0
            || next_pos.0 >= height as i32
            || next_pos.1 < 0
            || next_pos.1 >= width as i32
        {
            break;
        }

        if map[next_pos.0 as usize][next_pos.1 as usize] == b'#' {
            dir = match dir {
                UP => RIGHT,
                DOWN => LEFT,
                LEFT => UP,
                RIGHT => DOWN,
                _ => RIGHT,
            };

            if loop_detector.add((next_pos.0 as usize, next_pos.1 as usize)) {
                return None;
            }
        } else {
            pos = next_pos;
        }
    }

    Some(count)
}

fn find_all_obstacles(map: &Vec<Vec<u8>>) -> u64 {
    let height = map.len();
    let width = map[0].len();

    let mut count = 0;

    for i in 0..height {
        for j in 0..width {
            if map[i][j] == b'.' {
                let mut m = map.clone();
                m[i][j] = b'#';
                if walk(&mut m).is_none() {
                    // Loop
                    // println!("Found Obstacle {i} {j}");
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let input_file = "input.txt";
    let map: Vec<Vec<u8>> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    println!("Distinct Cells: {}", walk(&mut map.clone()).unwrap());
    println!("Obstacles: {}", find_all_obstacles(&map));
}
