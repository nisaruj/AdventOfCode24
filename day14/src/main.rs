use regex::Regex;
use std::fs;

const BOUNDARY: (i64, i64) = (101, 103);

#[derive(Debug, Clone)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    fn run(&mut self, t: i64, boundary: (i64, i64)) {
        self.position.0 = (self.position.0 + self.velocity.0 * t) % boundary.0;
        self.position.1 = (self.position.1 + self.velocity.1 * t) % boundary.1;

        if self.position.0 < 0 {
            self.position.0 = boundary.0 + self.position.0;
        }

        if self.position.1 < 0 {
            self.position.1 = boundary.1 + self.position.1;
        }
    }
}

fn calculate_safety_factor(robots: &Vec<Robot>, boundary: (i64, i64)) -> u64 {
    let mut quadrants: [[u64; 2]; 2] = [[0; 2]; 2];
    let middle = (boundary.0 / 2, boundary.1 / 2);

    for robot in robots {
        if robot.position.0 != middle.0 && robot.position.1 != middle.1 {
            quadrants[(robot.position.0 < middle.0) as usize]
                [(robot.position.1 < middle.1) as usize] += 1;
        }
    }

    quadrants[0][0] * quadrants[0][1] * quadrants[1][0] * quadrants[1][1]
}

fn log_if_possible_easter_egg(robots: &Vec<Robot>, file_name: &str) -> std::io::Result<()> {
    let mut map: [[bool; BOUNDARY.0 as usize]; BOUNDARY.1 as usize] =
        [[false; BOUNDARY.0 as usize]; BOUNDARY.1 as usize];
    let mut is_easter_egg = false;

    for robot in robots {
        map[robot.position.1 as usize][robot.position.0 as usize] = true;
    }

    let mut out: Vec<u8> = Vec::new();
    for i in 0..BOUNDARY.1 as usize {
        for j in 0..BOUNDARY.0 as usize {
            out.push(if map[i][j] { b'#' } else { b'-' });
            if map[i][j]
                && i + 2 < BOUNDARY.1 as usize
                && j + 2 < BOUNDARY.0 as usize
                && i >= 2
                && j >= 2
            {
                // The top of christmas tree should look like ^ shape
                if map[i + 1][j - 1] && map[i + 1][j + 1] && map[i + 2][j - 2] && map[i + 2][j + 2]
                {
                    // Possible Easter Egg
                    is_easter_egg = true;
                }
            }
        }
        out.push(b'\n');
    }

    if is_easter_egg {
        fs::write(file_name, out)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let input_file = "input.txt";
    let re = Regex::new(r"p=(\d+),(\d+)\sv=(-?\d+),(-?\d+)").unwrap();

    let input_str = fs::read_to_string(input_file).unwrap();

    let robots: Vec<Robot> = re
        .captures_iter(input_str.as_str())
        .map(|caps| {
            let (_, extracted) = caps.extract();
            let [p_x, p_y, v_x, v_y] = extracted.map(|s| s.parse::<i64>().unwrap());

            Robot {
                position: (p_x, p_y),
                velocity: (v_x, v_y),
            }
        })
        .collect();

    for robot in &mut robots.clone() {
        robot.run(100, BOUNDARY);
    }

    println!(
        "Safety Factor: {}",
        calculate_safety_factor(&robots, BOUNDARY)
    );

    for i in 0..10000 {
        let mut tmp: Vec<Robot> = robots.clone();
        for robot in &mut tmp {
            robot.run(i, BOUNDARY);
        }
        // Easter Egg is at 7412
        log_if_possible_easter_egg(&tmp, ["out/", i.to_string().as_str()].join("").as_str())?;
    }

    Ok(())
}
