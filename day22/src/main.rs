use std::collections::HashMap;

type PriceMap = HashMap<(u64, i8, i8, i8, i8), i8>;

struct PRNG {
    secret: u64,
}

impl PRNG {
    fn new(secret: u64) -> PRNG {
        PRNG { secret }
    }

    fn mix(&mut self, value: u64) {
        self.secret ^= value;
    }

    fn prune(&mut self) {
        self.secret = self.secret % 16777216;
    }

    fn next(&mut self) -> u64 {
        let result = self.secret * 64;
        self.mix(result);
        self.prune();
        let result = self.secret / 32;
        self.mix(result);
        self.prune();
        let result = self.secret * 2048;
        self.mix(result);
        self.prune();

        self.secret
    }
}

fn solve(initial_seeds: &Vec<u64>, price_map: &PriceMap) -> (u64, Vec<(i8, i8, i8, i8)>) {
    let mut best_price: u64 = 0;
    let mut best_sequence: Vec<(i8, i8, i8, i8)> = Vec::new();

    for i1 in -9..10 {
        for i2 in -9..10 {
            for i3 in -9..10 {
                for i4 in -9..10 {
                    let mut total_price: u64 = 0;
                    for seed in initial_seeds.iter() {
                        if !price_map.contains_key(&(*seed, i1, i2, i3, i4)) {
                            continue;
                        }
                        total_price += price_map[&(*seed, i1, i2, i3, i4)] as u64;
                    }
                    if total_price > best_price {
                        best_price = total_price;
                        best_sequence = vec![(i1, i2, i3, i4)];
                    }
                }
            }
        }
    }

    (best_price, best_sequence)
}

fn main() {
    let input_file = "input.txt";
    let mut sum = 0;

    let mut price_map: PriceMap = HashMap::new(); // initial seed + 4 consecutive changes => first occurrence of price
    let mut initial_seeds: Vec<u64> = Vec::new();

    for line in std::fs::read_to_string(input_file).unwrap().lines() {
        let seed: u64 = line.parse().unwrap();
        initial_seeds.push(seed);

        let mut prng = PRNG::new(seed);

        let mut prev_price: i8 = (seed % 10) as i8;
        let mut price_changes: Vec<i8> = Vec::new();

        for _ in 1..2000 {
            let price = (prng.next() % 10) as i8;
            price_changes.push(price - prev_price);

            if price_changes.len() > 4 {
                price_changes.remove(0);
            }

            if price_changes.len() == 4 {
                if !price_map.contains_key(&(
                    seed,
                    price_changes[0],
                    price_changes[1],
                    price_changes[2],
                    price_changes[3],
                )) {
                    price_map.insert(
                        (
                            seed,
                            price_changes[0],
                            price_changes[1],
                            price_changes[2],
                            price_changes[3],
                        ),
                        price,
                    );
                }
            }
            prev_price = price;
        }

        sum += prng.secret;
    }

    println!("Sum: {}", sum);

    let (best_price, best_sequence) = solve(&initial_seeds, &price_map);
    println!("Best price: {:?} {:?}", best_price, best_sequence);
}
