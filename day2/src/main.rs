use std::fs;

fn _is_safe(arr: &[u64], desc: bool) -> bool {
    let len = arr.len();

    if len <= 1 {
        return true;
    }

    let mut prev = arr[0];
    let mut i: usize = 1;

    while i < len {
        if desc {
            if prev <= arr[i] || prev - arr[i] > 3 {
                return false;
            }
        } else {
            if prev >= arr[i] || arr[i] - prev > 3 {
                return false;
            }
        }

        prev = arr[i];
        i += 1;
    }

    true
}

fn is_safe(arr: &[u64], tolerant: bool) -> bool {
    if tolerant {
        let len = arr.len();
        for i in 0..len {
            let mut temp = arr.to_vec();
            temp.remove(i);
            if _is_safe(&temp, false) || _is_safe(&temp, true) {
                return true;
            }
        }
    } else {
        return _is_safe(&arr, false) || _is_safe(&arr, true);
    }

    false
}

fn main() {
    let filename = "input.txt";
    let mut safe_count = (0, 0);

    for line in fs::read_to_string(filename).unwrap().lines() {
        let arr: Vec<u64> = line
            .split(" ")
            .map(str::parse::<u64>)
            .map(|x| x.unwrap())
            .collect();

        // println!("{}", is_safe(&arr, true));

        safe_count.0 += is_safe(&arr, false) as u64;
        safe_count.1 += is_safe(&arr, true) as u64;
    }

    println!("Safe Reports: {:?}", safe_count);
}
