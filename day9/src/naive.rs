use std::fs;

#[derive(Debug)]
struct Disk {
    map: Vec<Option<usize>>,
}

fn expand(disk_map: &str) -> Disk {
    let mut map: Vec<Option<usize>> = Vec::new();

    for (i, s) in disk_map.chars().enumerate() {
        let size = s.to_string().parse::<usize>().unwrap();

        if i % 2 == 0 {
            map.extend(std::iter::repeat(Some(i / 2)).take(size));
        } else {
            map.extend(std::iter::repeat(None).take(size));
        }
    }

    Disk { map }
}

fn rearrange(disk: &mut Disk) -> usize {
    let mut start: usize = 0;
    let mut end: usize = disk.map.len() - 1;
    let mut checksum: usize = 0;

    loop {
        while start < end && disk.map[end].is_none() {
            end -= 1;
        }

        while start < end && disk.map[start].is_some() {
            checksum += disk.map[start].unwrap() * start;
            start += 1;
        }

        if start >= end {
            break;
        }

        disk.map[start] = disk.map[end];
        disk.map[end] = None;
    }

    // Add the rest
    while disk.map[start].is_some() {
        checksum += disk.map[start].unwrap() * start;
        start += 1;
    }

    checksum
}

fn main() {
    let input_file = "test.txt";
    let disk_map = fs::read_to_string(input_file).unwrap();
    let mut disk = expand(&disk_map.trim());

    println!("Checksum {}", rearrange(&mut disk));
}
