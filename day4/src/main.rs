use std::fs;

// (vertical_offset, horizontal_offset, value)
const ALIGNMENTS: [[(i64, i64, u8); 4]; 8] = [
    [(0, 0, b'X'), (0, 1, b'M'), (0, 2, b'A'), (0, 3, b'S')], // Horizontal
    [(0, 0, b'X'), (0, -1, b'M'), (0, -2, b'A'), (0, -3, b'S')], // Horizontal Backward
    [(0, 0, b'X'), (1, 0, b'M'), (2, 0, b'A'), (3, 0, b'S')], // Vertical
    [(0, 0, b'X'), (-1, 0, b'M'), (-2, 0, b'A'), (-3, 0, b'S')], // Vertical Backward
    [(0, 0, b'X'), (1, 1, b'M'), (2, 2, b'A'), (3, 3, b'S')], // Diagonal DR
    [(0, 0, b'X'), (-1, -1, b'M'), (-2, -2, b'A'), (-3, -3, b'S')], // Diagonal UL
    [(0, 0, b'X'), (-1, 1, b'M'), (-2, 2, b'A'), (-3, 3, b'S')], // Diagonal UR
    [(0, 0, b'X'), (1, -1, b'M'), (2, -2, b'A'), (3, -3, b'S')], // Diagonal DL
];

const X_ALIGNMENTS: [[(i64, i64, u8); 5]; 4] = [
    [(0, 0, b'A'), (-1, -1, b'M'), (-1, 1, b'M'), (1, -1, b'S'), (1, 1, b'S')], // Top M M
    [(0, 0, b'A'), (-1, -1, b'M'), (-1, 1, b'S'), (1, -1, b'M'), (1, 1, b'S')], // Top M S
    [(0, 0, b'A'), (-1, -1, b'S'), (-1, 1, b'M'), (1, -1, b'S'), (1, 1, b'M')], // Top M M
    [(0, 0, b'A'), (-1, -1, b'S'), (-1, 1, b'S'), (1, -1, b'M'), (1, 1, b'M')], // Top S S
];

enum AlignmentType {
    Xmas = 0,
    CrossMas = 1,
}

struct Matrix {
    arr: Vec<Vec<u8>>,
}

impl Matrix {
    fn is_value_matched(&self, i: i64, j: i64, value: u8) -> bool {
        let height = self.arr.len();
        let width = self.arr.len();

        i >= 0
            && j >= 0
            && (i as usize) < height
            && (j as usize) < width
            && self.arr[i as usize][j as usize] == value
    }

    fn alignment_match(&self, alignment: &[(i64, i64, u8)], start_i: i64, start_j: i64) -> bool {
        for m in alignment {
            let (offset_i, offset_j, value) = m;
            if !self.is_value_matched(start_i + offset_i, start_j + offset_j, *value) {
                return false;
            }
        }
        true
    }

    fn find_all(&self, atype: AlignmentType) -> u64 {
        let height = self.arr.len();
        let width = self.arr.len();

        let mut count = 0;
        for i in 0..height {
            for j in 0..width {
                match atype {
                    AlignmentType::Xmas => {
                        for alignment in ALIGNMENTS {
                            if self.alignment_match(&alignment, i as i64, j as i64) {
                                count += 1;
                            }
                        }
                    },
                    AlignmentType::CrossMas => {
                        for alignment in X_ALIGNMENTS {
                            if self.alignment_match(&alignment, i as i64, j as i64) {
                                count += 1;
                            }
                        }
                    },
                }
            }
        }

        count
    }
}

fn main() {
    let input_file = "input.txt";

    let mut matrix = Matrix { arr: Vec::new() };

    for line in fs::read_to_string(input_file).unwrap().lines() {
        matrix.arr.push(line.as_bytes().to_vec());
    }

    println!("XMAS Found: {}", &matrix.find_all(AlignmentType::Xmas));
    println!("X-MAS Found: {}", &matrix.find_all(AlignmentType::CrossMas));
}
