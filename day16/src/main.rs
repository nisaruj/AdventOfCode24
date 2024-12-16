use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::{cmp::Reverse, fs};

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
enum Rotation {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

const DIRECTIONS: [(i32, i32, Rotation); 4] = [
    (-1, 0, Rotation::UP),
    (1, 0, Rotation::DOWN),
    (0, -1, Rotation::LEFT),
    (0, 1, Rotation::RIGHT),
];

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct MapPosition {
    i: usize,
    j: usize,
    rotation: Rotation,
    score: u64,
}

fn rev_direction(rot: Rotation) -> Rotation {
    match rot {
        Rotation::DOWN => Rotation::UP,
        Rotation::LEFT => Rotation::RIGHT,
        Rotation::UP => Rotation::DOWN,
        Rotation::RIGHT => Rotation::LEFT,
    }
}

fn backtrack(min_score: &Vec<Vec<Vec<u64>>>, curr_pos: MapPosition) -> HashSet<(usize, usize)> {
    let mut tiles: HashSet<(usize, usize)> = HashSet::from_iter([(curr_pos.i, curr_pos.j)]);

    if min_score[Rotation::RIGHT as usize][curr_pos.i][curr_pos.j] == 0 {
        return tiles;
    }

    let current_min_score = min_score[curr_pos.rotation.clone() as usize][curr_pos.i][curr_pos.j];

    for (i, j, rot) in DIRECTIONS {
        let new_i = (curr_pos.i as i32 + i) as usize;
        let new_j = (curr_pos.j as i32 + j) as usize;

        // Direct walk
        let prev_rot = rev_direction(rot.clone());
        let prev_min_score = min_score[prev_rot.clone() as usize][new_i][new_j];
        if prev_rot == curr_pos.rotation.clone()
            && prev_min_score != u64::MAX
            && current_min_score == prev_min_score + 1
        {
            let prev_tiles = backtrack(
                min_score,
                MapPosition {
                    i: new_i,
                    j: new_j,
                    rotation: prev_rot,
                    score: prev_min_score,
                },
            );
            tiles.extend(prev_tiles);
        }

        // 90 deg turn
        let prev_rots = if rot.clone() == Rotation::LEFT || rot.clone() == Rotation::RIGHT {
            [Rotation::DOWN, Rotation::UP]
        } else {
            [Rotation::LEFT, Rotation::RIGHT]
        };
        for prev_rot in prev_rots {
            let prev_min_score = min_score[prev_rot.clone() as usize][new_i][new_j];
            if prev_min_score != u64::MAX && current_min_score == prev_min_score + 1001 {
                let prev_tiles = backtrack(
                    min_score,
                    MapPosition {
                        i: new_i,
                        j: new_j,
                        rotation: prev_rot,
                        score: prev_min_score,
                    },
                );
                tiles.extend(prev_tiles);
            }
        }
    }

    tiles
}

fn find_n_tiles(min_score: &Vec<Vec<Vec<u64>>>, last_pos: (usize, usize)) -> usize {
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();

    for (_, _, rotation) in DIRECTIONS {
        tiles.extend(backtrack(
            min_score,
            MapPosition {
                i: last_pos.0,
                j: last_pos.1,
                rotation: rotation.clone(),
                score: min_score[rotation.clone() as usize][last_pos.0][last_pos.1],
            },
        ));
    }

    tiles.len()
}

fn bfs(map: &Vec<Vec<u8>>, start: (usize, usize, Rotation)) -> Option<(u64, usize)> {
    let height = map.len();
    let width = map[0].len();

    let mut min_score: Vec<Vec<Vec<u64>>> = vec![vec![vec![u64::MAX; width]; height]; 4];

    let mut pq: PriorityQueue<MapPosition, Reverse<u64>> = PriorityQueue::new();
    pq.push(
        MapPosition {
            i: start.0,
            j: start.1,
            rotation: start.2,
            score: 0,
        },
        Reverse(0),
    );

    while !pq.is_empty() {
        let (current_position, _) = pq.pop().unwrap();

        let rot_index = current_position.rotation.clone() as usize;

        if min_score[rot_index][current_position.i][current_position.j] > current_position.score {
            min_score[rot_index][current_position.i][current_position.j] = current_position.score;
        }

        if map[current_position.i][current_position.j] == b'E' {
            let n_tiles = find_n_tiles(&min_score, (current_position.i, current_position.j));
            return Some((current_position.score, n_tiles));
        }

        for (i, j, rot) in DIRECTIONS {
            let new_score;
            if rot.clone() == current_position.rotation.clone() {
                new_score = current_position.score + 1;
            }
            // 90 deg turn
            else if (rot.clone() as i32 % 2) != (current_position.rotation.clone() as i32 % 2) {
                new_score = current_position.score + 1001;
            }
            // 180 deg turn not allowed
            else {
                continue;
            }

            let new_i = (current_position.i as i32 + i) as usize;
            let new_j = (current_position.j as i32 + j) as usize;

            if map[new_i][new_j] == b'.' || map[new_i][new_j] == b'E' {
                let new_pos = MapPosition {
                    i: new_i,
                    j: new_j,
                    rotation: rot.clone(),
                    score: new_score,
                };

                pq.push(new_pos, Reverse(new_score));
            }
        }
    }

    None
}

fn main() {
    let input_file = "input.txt";

    let map: Vec<Vec<u8>> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    let start = (map.len() - 2, 1, Rotation::RIGHT);

    println!("{:?}", bfs(&map, start));
}
