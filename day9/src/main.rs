use std::collections::VecDeque;
use std::fs;

#[derive(Debug)]
struct DiskChunk {
    id: Option<usize>,
    size: usize,
    start: usize,
}

#[derive(Debug)]
struct Disk {
    free_chunks: VecDeque<DiskChunk>,
    alloc_chunks: Vec<DiskChunk>,
    used: usize,
}

impl DiskChunk {
    fn get_chunk_checksum(&self) -> Option<usize> {
        if self.id.is_some() {
            let checksum =
                ((self.size * (self.start + (self.start + self.size - 1))) / 2) * self.id.unwrap();
            return Some(checksum);
        }

        None
    }
}

fn expand(disk_map: &str) -> Disk {
    let mut disk: Disk = Disk {
        free_chunks: VecDeque::new(),
        alloc_chunks: Vec::new(),
        used: 0,
    };
    let mut index = 0;

    for (i, s) in disk_map.chars().enumerate() {
        let size = s.to_string().parse::<usize>().unwrap();

        if i % 2 == 0 {
            disk.alloc_chunks.push(DiskChunk {
                id: Some(i / 2),
                size,
                start: index,
            });
            disk.used += size;
        } else {
            disk.free_chunks.push_back(DiskChunk {
                id: None,
                size,
                start: index,
            });
        }

        index += size;
    }

    disk
}

fn rearranged_checksum(disk: &mut Disk, chunk_mode: bool) -> usize {
    let mut checksum: usize = 0;

    while !disk.free_chunks.is_empty() && !disk.alloc_chunks.is_empty() {
        if disk.free_chunks.front().unwrap().start >= disk.alloc_chunks.last().unwrap().start {
            break;
        }

        let mut free_chunk = disk.free_chunks.pop_front().unwrap();
        let mut alloc_chunk = disk.alloc_chunks.pop().unwrap();

        if alloc_chunk.size > free_chunk.size {
            // Split is not allowed in chunk mode
            if chunk_mode {
                // Find first-fit
                let mut found = false;
                disk.free_chunks.push_front(free_chunk);
                for i in 0..disk.free_chunks.len() {
                    if disk.free_chunks[i].start > alloc_chunk.start {
                        break;
                    }
                    if disk.free_chunks[i].size >= alloc_chunk.size {
                        let new_chunk = DiskChunk {
                            id: alloc_chunk.id,
                            size: alloc_chunk.size,
                            start: disk.free_chunks[i].start,
                        };
                        checksum += new_chunk.get_chunk_checksum().unwrap();

                        if disk.free_chunks[i].size == alloc_chunk.size {
                            disk.free_chunks.remove(i);
                        } else {
                            disk.free_chunks[i].size -= alloc_chunk.size;
                            disk.free_chunks[i].start += alloc_chunk.size;
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    checksum += alloc_chunk.get_chunk_checksum().unwrap();
                }
            } else {
                // Split Alloc Chunk
                let new_chunk = DiskChunk {
                    id: alloc_chunk.id,
                    size: free_chunk.size,
                    start: free_chunk.start,
                };
                checksum += new_chunk.get_chunk_checksum().unwrap();

                alloc_chunk.size -= free_chunk.size;
                disk.alloc_chunks.push(alloc_chunk);
            }
        } else if alloc_chunk.size < free_chunk.size {
            // Split Free Chunk
            let new_chunk = DiskChunk {
                id: alloc_chunk.id,
                size: alloc_chunk.size,
                start: free_chunk.start,
            };
            checksum += new_chunk.get_chunk_checksum().unwrap();

            free_chunk.start += alloc_chunk.size;
            free_chunk.size -= alloc_chunk.size;
            disk.free_chunks.push_front(free_chunk);
        } else {
            // Exactly same size, no need to split
            let new_chunk = DiskChunk {
                id: alloc_chunk.id,
                size: alloc_chunk.size,
                start: free_chunk.start,
            };
            checksum += new_chunk.get_chunk_checksum().unwrap();
        }
    }

    while !disk.alloc_chunks.is_empty() {
        let chunk = disk.alloc_chunks.pop().unwrap();
        checksum += chunk.get_chunk_checksum().unwrap();
    }

    checksum
}

fn main() {
    let input_file = "input.txt";
    let disk_map = fs::read_to_string(input_file).unwrap();
    let mut disk = expand(&disk_map.trim());
    println!("Checksum {}", rearranged_checksum(&mut disk, false));

    let disk_map = fs::read_to_string(input_file).unwrap();
    let mut disk = expand(&disk_map.trim());
    println!("Checksum {}", rearranged_checksum(&mut disk, true));
}
