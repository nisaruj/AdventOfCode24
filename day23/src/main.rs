use std::collections::{HashMap, HashSet};
use std::fs;

fn part1(computer_vec: &Vec<String>, computer_map: &HashSet<(String, String)>) -> usize {
    let mut triplets: Vec<Vec<String>> = Vec::new();

    for comp1 in 0..computer_vec.len() {
        for comp2 in comp1 + 1..computer_vec.len() {
            for comp3 in comp2 + 1..computer_vec.len() {
                if computer_map
                    .contains(&(computer_vec[comp1].clone(), computer_vec[comp2].clone()))
                    && computer_map
                        .contains(&(computer_vec[comp2].clone(), computer_vec[comp3].clone()))
                    && computer_map
                        .contains(&(computer_vec[comp3].clone(), computer_vec[comp1].clone()))
                {
                    if computer_vec[comp1].starts_with("t")
                        || computer_vec[comp2].starts_with("t")
                        || computer_vec[comp3].starts_with("t")
                    {
                        triplets.push(vec![
                            computer_vec[comp1].clone(),
                            computer_vec[comp2].clone(),
                            computer_vec[comp3].clone(),
                        ]);
                    }
                }
            }
        }
    }

    triplets.len()
}

// Find largest complete subgraph
fn bron_kerbosch(
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    adj_list: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    while !p.is_empty() {
        let v = p.iter().next().unwrap().clone();
        let mut r_new = r.clone();
        r_new.insert(v.clone());
        let mut p_new: HashSet<String> = p.intersection(&adj_list[&v]).cloned().collect();
        let mut x_new: HashSet<String> = x.intersection(&adj_list[&v]).cloned().collect();
        bron_kerbosch(&mut r_new, &mut p_new, &mut x_new, adj_list, cliques);
        p.remove(&v);
        x.insert(v);
    }
}

fn part2(
    adj_list: &HashMap<String, HashSet<String>>,
    vertices: &HashSet<String>,
) -> HashSet<String> {
    let mut r: HashSet<String> = HashSet::new();
    let mut p: HashSet<String> = vertices.clone();
    let mut x: HashSet<String> = HashSet::new();
    let mut cliques: Vec<HashSet<String>> = Vec::new();

    bron_kerbosch(&mut r, &mut p, &mut x, adj_list, &mut cliques);

    let mut max_clique = HashSet::new();
    for clique in cliques {
        if clique.len() > max_clique.len() {
            max_clique = clique;
        }
    }

    max_clique
}

fn main() {
    let input_file = "input.txt";

    let mut computer_map: HashSet<(String, String)> = HashSet::new();
    let mut computers: HashSet<String> = HashSet::new();
    let mut adj_list: HashMap<String, HashSet<String>> = HashMap::new();

    for line in fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
    {
        let mut parts = line.split("-");
        let comp1 = parts.next().unwrap().to_string();
        let comp2: String = parts.next().unwrap().to_string();

        computers.insert(comp1.clone());
        computers.insert(comp2.clone());
        computer_map.insert((comp1.clone(), comp2.clone()));
        computer_map.insert((comp2.clone(), comp1.clone()));

        // Add to adjacency list
        if !adj_list.contains_key(&comp1) {
            adj_list.insert(comp1.clone(), HashSet::new());
        }
        adj_list.get_mut(&comp1).unwrap().insert(comp2.clone());
        if !adj_list.contains_key(&comp2) {
            adj_list.insert(comp2.clone(), HashSet::new());
        }
        adj_list.get_mut(&comp2).unwrap().insert(comp1.clone());
    }

    let computer_vec: Vec<String> = computers.clone().into_iter().collect();

    println!("Number of computers: {}", computer_vec.len());
    println!(
        "Number of part 1 triplets: {}",
        part1(&computer_vec, &computer_map)
    );

    let max_clique = part2(&adj_list, &computers);
    println!("Max Clique {:?} ({})", max_clique, max_clique.len());
    let mut sorted_clique = max_clique.into_iter().collect::<Vec<String>>();
    sorted_clique.sort();
    let password = sorted_clique.join(",");
    println!("Password: {}", password);
}
