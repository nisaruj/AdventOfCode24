use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    WALL,
    BOX,
    BLANK,
    PLAYER,
    DBoxL,
    DBoxR,
}

trait MapSimulation {
    fn simulate(&mut self, direction: Move) -> (usize, usize);
    fn boxes_sum(&self) -> usize;

    #[allow(dead_code)]
    fn print(&self);
}

struct NormalMap {
    map: Vec<Vec<Cell>>,
    player: (usize, usize),
}

#[derive(Debug, Clone)]
struct DoubleWideMap {
    map: Vec<Vec<Cell>>,
    player: (usize, usize),
}

impl DoubleWideMap {
    fn from_normal_map(map: &NormalMap) -> DoubleWideMap {
        let mut dmap: Vec<Vec<Cell>> = Vec::new();

        for row in &map.map {
            let mut drow: Vec<Cell> = Vec::new();
            for cell in row {
                match &cell {
                    Cell::BOX => {
                        drow.push(Cell::DBoxL);
                        drow.push(Cell::DBoxR);
                    }
                    Cell::PLAYER => {
                        drow.push(Cell::PLAYER);
                        drow.push(Cell::BLANK);
                    }
                    _ => {
                        drow.push(cell.clone());
                        drow.push(cell.clone());
                    }
                }
            }

            dmap.push(drow);
        }

        DoubleWideMap {
            map: dmap,
            player: (map.player.0, map.player.1 * 2),
        }
    }
}

impl MapSimulation for NormalMap {
    fn simulate(&mut self, direction: Move) -> (usize, usize) {
        assert_eq!(self.map[self.player.0][self.player.1], Cell::PLAYER);

        let dir: (i32, i32) = match direction {
            Move::UP => (-1, 0),
            Move::DOWN => (1, 0),
            Move::LEFT => (0, -1),
            Move::RIGHT => (0, 1),
        };

        let mut curr_pos = self.player;
        while self.map[curr_pos.0][curr_pos.1] != Cell::WALL {
            if self.map[curr_pos.0][curr_pos.1] == Cell::BLANK {
                // Not blocked by a wall, then shift
                let rev_dir = (dir.0 * -1, dir.1 * -1);

                while curr_pos.0 != self.player.0 || curr_pos.1 != self.player.1 {
                    let new_pos = (
                        (curr_pos.0 as i32 + rev_dir.0) as usize,
                        (curr_pos.1 as i32 + rev_dir.1) as usize,
                    );
                    self.map[curr_pos.0][curr_pos.1] = self.map[new_pos.0][new_pos.1].clone();
                    curr_pos = new_pos;
                }

                self.map[self.player.0][self.player.1] = Cell::BLANK;
                self.player = (
                    (self.player.0 as i32 + dir.0) as usize,
                    (self.player.1 as i32 + dir.1) as usize,
                );
                return self.player;
            }

            curr_pos.0 = (curr_pos.0 as i32 + dir.0) as usize;
            curr_pos.1 = (curr_pos.1 as i32 + dir.1) as usize;
        }

        self.player
    }

    fn boxes_sum(&self) -> usize {
        let mut sum = 0;

        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == Cell::BOX {
                    sum += i * 100 + j;
                }
            }
        }

        sum
    }

    fn print(&self) {
        for line in &self.map {
            for c in line {
                print!(
                    "{}",
                    match c {
                        Cell::BLANK => '.',
                        Cell::BOX => 'O',
                        Cell::PLAYER => '@',
                        Cell::WALL => '#',
                        Cell::DBoxL => '[',
                        Cell::DBoxR => ']',
                    }
                );
            }
            print!("\n");
        }
    }
}

impl DoubleWideMap {
    // Find all affected boxes in case of UP/DOWN move
    // Return None if the move is impossible
    fn affected_boxes(
        &self,
        lbox_pos: (usize, usize),
        direction: Move,
    ) -> Option<Vec<(usize, usize)>> {
        let vertical_dir: i32 = if direction == Move::UP { -1 } else { 1 };
        let vertically_next_lpos = (lbox_pos.0 as i32 + vertical_dir) as usize;

        let mut all_affected_boxes: Vec<(usize, usize)> = vec![lbox_pos];

        // If the cells above the box are blank, then this move might be possible
        if self.map[vertically_next_lpos][lbox_pos.1] == Cell::BLANK
            && self.map[vertically_next_lpos][lbox_pos.1 + 1] == Cell::BLANK
        {
            return Some(all_affected_boxes);
        }
        // If a box faces a wall, then this move is impossible
        else if self.map[vertically_next_lpos][lbox_pos.1] == Cell::WALL
            || self.map[vertically_next_lpos][lbox_pos.1 + 1] == Cell::WALL
        {
            // Impossible Move
            return None;
        }
        // The above box is directly above/below the current box
        else if self.map[vertically_next_lpos][lbox_pos.1] == Cell::DBoxL {
            // Propagate New Box
            let boxes = self.affected_boxes((vertically_next_lpos, lbox_pos.1), direction);
            if boxes.is_none() {
                return None;
            } else {
                all_affected_boxes.extend(boxes.unwrap());
            }
        }
        // If above/below box is a bit off
        else {
            // Propagate New Box on the left
            if self.map[vertically_next_lpos][lbox_pos.1] == Cell::DBoxR {
                let boxes =
                    self.affected_boxes((vertically_next_lpos, lbox_pos.1 - 1), direction.clone());
                if boxes.is_none() {
                    return None;
                } else {
                    all_affected_boxes.extend(boxes.unwrap());
                }
            }

            // Propagate New Box on the right
            if self.map[vertically_next_lpos][lbox_pos.1 + 1] == Cell::DBoxL {
                let boxes =
                    self.affected_boxes((vertically_next_lpos, lbox_pos.1 + 1), direction.clone());
                if boxes.is_none() {
                    return None;
                } else {
                    all_affected_boxes.extend(boxes.unwrap());
                }
            }
        }

        Some(all_affected_boxes)
    }
}

impl MapSimulation for DoubleWideMap {
    fn simulate(&mut self, direction: Move) -> (usize, usize) {
        assert_eq!(self.map[self.player.0][self.player.1], Cell::PLAYER);

        let dir: (i32, i32) = match direction {
            Move::UP => (-1, 0),
            Move::DOWN => (1, 0),
            Move::LEFT => (0, -1),
            Move::RIGHT => (0, 1),
        };

        if direction == Move::LEFT || direction == Move::RIGHT {
            let mut curr_pos = self.player;
            while self.map[curr_pos.0][curr_pos.1] != Cell::WALL {
                if self.map[curr_pos.0][curr_pos.1] == Cell::BLANK {
                    // Not blocked by a wall, then shift
                    let rev_dir = (dir.0 * -1, dir.1 * -1);

                    while curr_pos.0 != self.player.0 || curr_pos.1 != self.player.1 {
                        let new_pos = (
                            (curr_pos.0 as i32 + rev_dir.0) as usize,
                            (curr_pos.1 as i32 + rev_dir.1) as usize,
                        );
                        self.map[curr_pos.0][curr_pos.1] = self.map[new_pos.0][new_pos.1].clone();
                        curr_pos = new_pos;
                    }

                    self.map[self.player.0][self.player.1] = Cell::BLANK;
                    self.player = (
                        (self.player.0 as i32 + dir.0) as usize,
                        (self.player.1 as i32 + dir.1) as usize,
                    );
                    return self.player;
                }

                curr_pos.0 = (curr_pos.0 as i32 + dir.0) as usize;
                curr_pos.1 = (curr_pos.1 as i32 + dir.1) as usize;
            }
        } else {
            let next_pos = (
                (self.player.0 as i32 + dir.0) as usize,
                (self.player.1 as i32 + dir.1) as usize,
            );

            if self.map[next_pos.0][next_pos.1] == Cell::BLANK {
                self.map[self.player.0][self.player.1] = Cell::BLANK;
                self.map[next_pos.0][next_pos.1] = Cell::PLAYER;

                self.player = next_pos;
            } else if self.map[next_pos.0][next_pos.1] != Cell::WALL {
                // The player faces boxes
                let lbox_pos = if self.map[next_pos.0][next_pos.1] == Cell::DBoxL {
                    next_pos
                } else {
                    (next_pos.0, next_pos.1 - 1)
                };

                let boxes = self.affected_boxes(lbox_pos, direction.clone());

                if boxes.is_some() {
                    let mut boxes_to_be_moved = boxes.unwrap();
                    // If dir = UP, move the boxes from the most top first
                    boxes_to_be_moved.sort();

                    if direction == Move::DOWN {
                        // If dir = DOWN, move the boxes from the bottom first
                        boxes_to_be_moved.reverse();
                    }

                    for (i, j) in boxes_to_be_moved {
                        let new_box_pos =
                            ((i as i32 + dir.0) as usize, (j as i32 + dir.1) as usize);

                        self.map[new_box_pos.0][new_box_pos.1] = Cell::DBoxL;
                        self.map[new_box_pos.0][new_box_pos.1 + 1] = Cell::DBoxR;
                        self.map[i][j] = Cell::BLANK;
                        self.map[i][j + 1] = Cell::BLANK;
                    }

                    self.map[self.player.0][self.player.1] = Cell::BLANK;
                    self.map[next_pos.0][next_pos.1] = Cell::PLAYER;

                    self.player = next_pos;
                }
            }
        }

        self.player
    }

    fn boxes_sum(&self) -> usize {
        let mut sum = 0;

        let height = self.map.len();
        let width = self.map[0].len();

        for i in 0..height {
            for j in 0..width {
                if self.map[i][j] == Cell::DBoxL {
                    let vertical_distance = i;
                    let horizontal_distance = j;

                    sum += vertical_distance * 100 + horizontal_distance;
                }
            }
        }

        sum
    }

    fn print(&self) {
        for line in &self.map {
            for c in line {
                print!(
                    "{}",
                    match c {
                        Cell::BLANK => '.',
                        Cell::BOX => 'O',
                        Cell::PLAYER => '@',
                        Cell::WALL => '#',
                        Cell::DBoxL => '[',
                        Cell::DBoxR => ']',
                    }
                );
            }
            print!("\n");
        }
    }
}

fn main() {
    let input_file = "input.txt";
    let mut map: NormalMap = NormalMap {
        map: Vec::new(),
        player: (0, 0),
    };
    let mut moves: Vec<Move> = Vec::new();

    let fs = fs::read_to_string(input_file).unwrap();
    let mut file_iter = fs.lines();

    let mut line = file_iter.next();

    while line.is_some() && !line.unwrap().is_empty() {
        let row: Vec<Cell> = line
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => Cell::WALL,
                'O' => Cell::BOX,
                '.' => Cell::BLANK,
                '@' => Cell::PLAYER,
                _ => Cell::WALL,
            })
            .collect();

        let player_pos = row.clone().into_iter().position(|c| c == Cell::PLAYER);
        if player_pos.is_some() {
            map.player = (map.map.len(), player_pos.unwrap());
        }

        map.map.push(row);
        line = file_iter.next();
    }

    line = file_iter.next();
    while line.is_some() && !line.unwrap().is_empty() {
        moves.extend(line.unwrap().chars().map(|c| match c {
            '^' => Move::UP,
            'v' => Move::DOWN,
            '<' => Move::LEFT,
            '>' => Move::RIGHT,
            _ => Move::UP,
        }));
        line = file_iter.next();
    }

    let mut dmap = DoubleWideMap::from_normal_map(&map);

    for m in &moves {
        map.simulate(m.clone());
    }
    println!("Normal Map: {}", map.boxes_sum());

    for m in &moves {
        dmap.simulate(m.clone());
    }

    println!("Double-Wide Map: {}", dmap.boxes_sum());
}
