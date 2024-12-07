use std::fs;

fn solve(target: u64, arr: &Vec<u64>, index: usize, sum: u64, concat_mode: bool) -> bool {
    if index >= arr.len() {
        return sum == target;
    }
    return solve(target, &arr, index + 1, sum + arr[index], concat_mode)
        || solve(target, &arr, index + 1, sum * arr[index], concat_mode)
        || (concat_mode
            && solve(
                target,
                &arr,
                index + 1,
                (sum.to_string() + arr[index].to_string().as_str())
                    .parse::<u64>()
                    .unwrap(),
                concat_mode,
            ));
}

fn main() {
    let input_file = "input.txt";
    let mut sum: (u64, u64) = (0, 0);

    for line in fs::read_to_string(input_file).unwrap().lines() {
        let mut parts = line.split(": ");
        let target = parts.next().unwrap().parse::<u64>().unwrap();
        let arr: Vec<u64> = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        println!("{} {:?}", target, arr);

        if solve(target, &arr, 1, arr[0], false) {
            sum.0 += target;
        }
        if solve(target, &arr, 1, arr[0], true) {
            sum.1 += target;
        }
    }

    println!("Sum = {} {}", sum.0, sum.1);
}
