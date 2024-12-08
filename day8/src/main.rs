use std::collections::{HashMap, HashSet};
use std::fs;

type Antinodes = HashSet<(usize, usize)>;
struct AntennaMap {
    antennas: HashMap<u8, Vec<(usize, usize)>>,
    size: (usize, usize),
}

impl AntennaMap {
    fn calculate_antinodes(
        &self,
        antenna_base: (usize, usize),
        antenna_another: (usize, usize),
        unlimited_mode: bool,
    ) -> Antinodes {
        let distance = (
            (antenna_base.0 as i32 - antenna_another.0 as i32).abs(),
            (antenna_base.1 as i32 - antenna_another.1 as i32).abs(),
        );

        let mut antinodes: Antinodes = if unlimited_mode {
            HashSet::from_iter([antenna_base])
        } else {
            HashSet::new()
        };
        let mut multipiler = 1;

        loop {
            let i: i32 = if antenna_base.0 < antenna_another.0 {
                antenna_base.0 as i32 - distance.0 * multipiler
            } else {
                antenna_base.0 as i32 + distance.0 * multipiler
            };

            let j: i32 = if antenna_base.1 < antenna_another.1 {
                antenna_base.1 as i32 - distance.1 * multipiler
            } else {
                antenna_base.1 as i32 + distance.1 * multipiler
            };

            if i >= 0 && i < self.size.0 as i32 && j >= 0 && j < self.size.1 as i32 {
                antinodes.insert((i as usize, j as usize));
                multipiler += 1;
            } else {
                break;
            }

            if !unlimited_mode {
                break;
            }
        }

        antinodes
    }

    fn find_all_antinodes(&self, unlimited_mode: bool) -> usize {
        let mut antinodes: Antinodes = HashSet::new();

        for freq in self.antennas.keys() {
            let antennas_freq = self.antennas.get(freq).unwrap();
            let len = antennas_freq.len();
            for i in 0..len {
                for j in 0..len {
                    if i != j {
                        let calculated_antinodes: Antinodes = self.calculate_antinodes(
                            antennas_freq[i],
                            antennas_freq[j],
                            unlimited_mode,
                        );

                        antinodes.extend(calculated_antinodes);
                    }
                }
            }
        }

        antinodes.len()
    }
}

fn main() {
    let input_file = "input.txt";
    let lines: Vec<String> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut map: AntennaMap = AntennaMap {
        antennas: HashMap::new(),
        size: (lines.len(), lines[0].len()),
    };

    for (i, line) in lines.into_iter().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if c != b'.' {
                if !map.antennas.contains_key(&c) {
                    map.antennas.insert(c, Vec::new());
                }
                map.antennas.get_mut(&c).unwrap().push((i, j));
            }
        }
    }

    println!(
        "Antinodes with limited distance: {}",
        map.find_all_antinodes(false)
    );
    println!(
        "Antinodes with unlimited distance: {}",
        map.find_all_antinodes(true)
    );
}
