use std::fs;

const N_NODE: usize = 105;
type Prerequisites = [[bool; N_NODE]; N_NODE];

fn validate_pages(prereq: &Prerequisites, pages: &Vec<usize>) -> Option<(usize, usize)> {
    for i in 0..pages.len() {
        for j in 0..i {
            if prereq[pages[j]][pages[i]] {
                return Some((i, j));
            }
        }
    }
    None
}

fn fix_pages(prereq: &Prerequisites, pages: &Vec<usize>) -> Vec<usize> {
    let mut fixed = pages.clone();

    let mut res = validate_pages(prereq, &fixed);
    while res.is_some() {
        let (i, j) = res.unwrap();
        let temp = fixed[i];
        fixed[i] = fixed[j];
        fixed[j] = temp;

        res = validate_pages(prereq, &fixed);
    }
    fixed
}

fn main() {
    let input_file = "input.txt";
    let fs = fs::read_to_string(input_file).unwrap();
    let mut line_iter = fs.lines();
    let mut line = line_iter.next();

    let mut prereq = [[false; N_NODE]; N_NODE];

    // Load Prerequisites
    while line.is_some() && line.unwrap().len() != 0 {
        let parts: Vec<&str> = line.unwrap().split("|").collect();

        let from = parts[0].parse::<usize>().unwrap();
        let to = parts[1].parse::<usize>().unwrap();

        prereq[to][from] = true;

        line = line_iter.next();
    }

    let mut midsum: usize = 0;
    let mut fixedsum: usize = 0;

    // Load Queries
    line = line_iter.next();
    while line.is_some() {
        let pages: Vec<usize> = line
            .unwrap()
            .split(",")
            .map(|p| p.parse::<usize>().unwrap())
            .collect();

        if validate_pages(&prereq, &pages).is_none() {
            midsum += pages[pages.len() / 2];
        } else {
            let fixed = fix_pages(&prereq, &pages);
            // println!("Fixed {:?}", fixed);
            fixedsum += fixed[fixed.len() / 2];
        }

        line = line_iter.next();
    }

    println!("Sum {}", midsum);
    println!("Fixed Sum {}", fixedsum);
}
