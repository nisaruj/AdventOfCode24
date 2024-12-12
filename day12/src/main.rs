use std::fs;

#[derive(Debug)]
enum FenceSide {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

const DIRECTIONS: [(i32, i32, FenceSide); 4] = [
    (1, 0, FenceSide::BOTTOM),
    (-1, 0, FenceSide::TOP),
    (0, 1, FenceSide::RIGHT),
    (0, -1, FenceSide::LEFT),
];

#[derive(Debug)]
struct Fence {
    side: FenceSide,
    pos: (i32, i32),
}

#[derive(Debug)]
struct PlotRequirements {
    area: usize,
    fences: Vec<Fence>,
}

type GardenRequirements = Vec<PlotRequirements>;
type GardenMap<T> = Vec<Vec<T>>;

fn get_fences(map: &GardenMap<u8>, pos: (usize, usize)) -> Vec<Fence> {
    let mut fences: Vec<Fence> = Vec::new();

    for dir in DIRECTIONS {
        let new_pos: (i32, i32) = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= map.len() as i32
            || new_pos.1 >= map[0].len() as i32
            || map[new_pos.0 as usize][new_pos.1 as usize] != map[pos.0][pos.1]
        {
            // Adjacent plant is not the same type, then add a fence
            fences.push(Fence {
                side: dir.2,
                pos: new_pos,
            });
        }
    }

    fences
}

fn flood_fill(
    map: &GardenMap<u8>,
    visited: &mut GardenMap<bool>,
    pos: (usize, usize),
) -> PlotRequirements {
    let height = map.len();
    let width = map[0].len();
    let plant_id = map[pos.0][pos.1];

    visited[pos.0][pos.1] = true;

    let mut req: PlotRequirements = PlotRequirements {
        area: 1,
        fences: get_fences(map, pos),
    };

    for dir in DIRECTIONS {
        let new_pos: (i32, i32) = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if new_pos.0 >= 0
            && new_pos.1 >= 0
            && new_pos.0 < height as i32
            && new_pos.1 < width as i32
            && !visited[new_pos.0 as usize][new_pos.1 as usize]
            && map[new_pos.0 as usize][new_pos.1 as usize] == plant_id
        {
            let result = flood_fill(map, visited, (new_pos.0 as usize, new_pos.1 as usize));
            req.area += result.area;
            req.fences.extend(result.fences);
        }
    }

    req
}

fn get_full_price(garden: GardenRequirements) -> u64 {
    garden.into_iter().fold(0, |price, req| {
        price + req.area as u64 * req.fences.len() as u64
    })
}

fn get_discounted_price(garden: GardenRequirements) -> u64 {
    let mut price: u64 = 0;

    for req in garden {
        let mut side_fences: [Vec<(i32, i32)>; 4] = Default::default();

        for fence in req.fences {
            match fence.side {
                FenceSide::LEFT => {
                    side_fences[0].push((fence.pos.1, fence.pos.0));
                }
                FenceSide::RIGHT => {
                    side_fences[1].push((fence.pos.1, fence.pos.0));
                }
                FenceSide::TOP => {
                    side_fences[2].push(fence.pos);
                }
                FenceSide::BOTTOM => {
                    side_fences[3].push(fence.pos);
                }
            }
        }

        let mut all_sides = 0;

        for i in 0..4 {
            side_fences[i].sort();

            let mut sides = 0;

            if !side_fences[i].is_empty() {
                sides += 1;
                for j in 0..(side_fences[i].len() - 1) {
                    if side_fences[i][j].0 != side_fences[i][j + 1].0
                        || side_fences[i][j].1 + 1 != side_fences[i][j + 1].1
                    {
                        // Next fence is not adjacent, count the next one as another separated side
                        sides += 1;
                    }
                }
            }

            all_sides += sides;
        }

        price += req.area as u64 * all_sides;
    }

    price
}

fn solve(map: &GardenMap<u8>, side_mode: bool) -> u64 {
    let height = map.len();
    let width = map[0].len();

    let mut visited: GardenMap<bool> = vec![vec![false; width]; height];
    let mut req: GardenRequirements = Vec::new();

    for i in 0..height {
        for j in 0..width {
            if !visited[i][j] {
                req.push(flood_fill(map, &mut visited, (i, j)));
            }
        }
    }

    return if side_mode {
        get_discounted_price(req)
    } else {
        get_full_price(req)
    };
}

fn main() {
    let input_file = "input.txt";

    let map: Vec<Vec<u8>> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    println!("Full Price: {}", solve(&map, false));
    println!("Discount Price: {}", solve(&map, true));
}
