use std::{fs, vec};

#[derive(Clone, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Push,
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
const NUMPAD_POSITIONS: [(usize, usize); 11] = [
    (3, 1), // 0
    (2, 0), // 1
    (2, 1), // 2
    (2, 2), // 3
    (1, 0), // 4
    (1, 1), // 5
    (1, 2), // 6
    (0, 0), // 7
    (0, 1), // 8
    (0, 2), // 9
    (3, 2), // A
];
const NUMPAD_GAPS: [(usize, usize); 1] = [(3, 0)];
const NUMPAD_A_BUTTON: (usize, usize) = (3, 2);

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
 */
const ROBOTPAD_POSITIONS: [(usize, usize); 5] = [
    (0, 1), // UP
    (1, 1), // DOWN
    (1, 0), // LEFT
    (1, 2), // RIGHT
    (0, 2), // A
];
const ROBOTPAD_GAPS: [(usize, usize); 1] = [(0, 0)];

// const PART1_DEPTH: usize = 2;
const PART2_DEPTH: usize = 25;

const ROBOT_DEPTH: usize = PART2_DEPTH;

struct RobotPad {
    position: (usize, usize),
    gaps: Vec<(usize, usize)>,
    depth: usize,
}

impl RobotPad {
    fn new(depth: usize) -> Self {
        RobotPad {
            position: (0, 2),
            gaps: ROBOTPAD_GAPS.to_vec(),
            depth,
        }
    }
}

fn simulate_robot_move(
    memo: &mut Vec<Vec<Vec<Option<usize>>>>,
    depth: usize,
    sequence: Vec<Direction>,
    current_depth: usize,
) -> usize {
    if depth == current_depth {
        return sequence.len();
    }

    let mut robot: RobotPad = RobotPad::new(current_depth);
    let mut sequence_length = 0;

    for robot_move in sequence {
        // Memoization Check
        if let Some(len) =
            memo[current_depth][robot.position.0 * 3 + robot.position.1][robot_move as usize]
        {
            sequence_length += len;
            robot.position = ROBOTPAD_POSITIONS[robot_move as usize];
            continue;
        }

        let prev_position = robot.position;

        // Find the minimum sequence length for deepr levels
        let seq_len = robot.move_and_push(memo, robot_move);

        sequence_length += seq_len;

        // Memoization
        memo[current_depth][prev_position.0 * 3 + prev_position.1][robot_move as usize] =
            Some(seq_len);
    }

    sequence_length
}

// Try all possible moves in current depth
fn _find_best_path(
    memo: &mut Vec<Vec<Vec<Option<usize>>>>,
    gaps: &Vec<(usize, usize)>,
    simulation_depth: usize,
    current_depth: usize,
    vertical_direction: i32,
    horizontal_direction: i32,
    sequence: Vec<Direction>,
    current_position: (usize, usize),
) -> usize {
    if vertical_direction == 0 && horizontal_direction == 0 {
        let mut final_sequence = sequence.clone();
        final_sequence.push(Direction::Push);

        // The sequence is ready, simulate find the length of the last controller
        let best_sequence_len =
            simulate_robot_move(memo, simulation_depth, final_sequence, current_depth);

        return best_sequence_len;
    }

    if gaps.contains(&current_position) {
        // Bad Cell not allowed
        return std::usize::MAX;
    }

    let mut best_length = std::usize::MAX;
    let mut new_sequence = sequence.clone();

    // Try vertical move
    if vertical_direction != 0 {
        new_sequence.push(if vertical_direction < 0 {
            Direction::Up
        } else {
            Direction::Down
        });

        let new_position = (
            (current_position.0 as i32 + vertical_direction.signum()) as usize,
            current_position.1,
        );

        let length = _find_best_path(
            memo,
            gaps,
            simulation_depth,
            current_depth,
            vertical_direction - vertical_direction.signum(),
            horizontal_direction,
            new_sequence,
            new_position,
        );

        if length < best_length {
            best_length = length;
        }
    }

    // Try horizontal move
    let mut new_sequence = sequence.clone();
    if horizontal_direction != 0 {
        new_sequence.push(if horizontal_direction < 0 {
            Direction::Left
        } else {
            Direction::Right
        });

        let new_position = (
            current_position.0,
            (current_position.1 as i32 + horizontal_direction.signum()) as usize,
        );

        let length = _find_best_path(
            memo,
            gaps,
            simulation_depth,
            current_depth,
            vertical_direction,
            horizontal_direction - horizontal_direction.signum(),
            new_sequence,
            new_position,
        );

        if length < best_length {
            best_length = length;
        }
    }

    best_length
}

// Wrapper function for find_best_path
fn find_best_path(
    memo: &mut Vec<Vec<Vec<Option<usize>>>>,
    gaps: &Vec<(usize, usize)>,
    current_depth: usize,
    target_position: (usize, usize),
    current_position: (usize, usize),
) -> usize {
    _find_best_path(
        memo,
        gaps,
        ROBOT_DEPTH,
        current_depth,
        target_position.0 as i32 - current_position.0 as i32,
        target_position.1 as i32 - current_position.1 as i32,
        vec![],
        current_position,
    )
}

impl RobotPad {
    fn move_and_push(
        &mut self,
        memo: &mut Vec<Vec<Vec<Option<usize>>>>,
        direction: Direction,
    ) -> usize {
        if self.depth == ROBOT_DEPTH {
            // Base case, no need to go deeper
            return 1;
        }

        let best_sequence_len = find_best_path(
            memo,
            &self.gaps,
            self.depth + 1,
            ROBOTPAD_POSITIONS[direction as usize],
            self.position,
        );

        self.position = ROBOTPAD_POSITIONS[direction as usize];

        best_sequence_len
    }
}

struct NumPad {
    position: (usize, usize),
    gaps: Vec<(usize, usize)>,

    // mem[depth][current_position][direction] => sequence length
    mem: Vec<Vec<Vec<Option<usize>>>>,
}

impl Default for NumPad {
    fn default() -> Self {
        NumPad {
            position: NUMPAD_A_BUTTON,
            gaps: NUMPAD_GAPS.to_vec(),
            mem: vec![vec![vec![None; 5]; 8]; 25],
        }
    }
}

impl NumPad {
    fn move_and_push(&mut self, num: usize) -> usize {
        let length = find_best_path(
            &mut self.mem,
            &self.gaps,
            0,
            NUMPAD_POSITIONS[num],
            self.position,
        );

        self.position = NUMPAD_POSITIONS[num];

        length
    }

    fn calculate_complexity(&mut self, line: &str) -> usize {
        let mut sum_length = 0;

        for c in line.chars() {
            let num = c.to_digit(16).unwrap() as usize;
            let l = self.move_and_push(num);
            sum_length += l;
        }

        let complexity = line
            .chars()
            .take(3)
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
            * sum_length;

        complexity
    }
}

fn main() {
    let mut numpad = NumPad::default();

    let input_file = "input.txt";

    let mut sum = 0;
    for line in fs::read_to_string(input_file).unwrap().lines() {
        sum += numpad.calculate_complexity(line);
    }

    println!("Part 2 Complexity {}", sum);
}
