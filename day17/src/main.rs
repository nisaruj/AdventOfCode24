struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    instructions: Vec<u8>,
    rip: usize,
}

impl Computer {
    fn new(reg_a: i64, reg_b: i64, reg_c: i64, instructions: Vec<u8>) -> Computer {
        Computer {
            reg_a,
            reg_b,
            reg_c,
            instructions,
            rip: 0,
        }
    }

    fn execute(&mut self) -> Vec<u8> {
        let mut outputs: Vec<u8> = Vec::new();

        while self.rip < self.instructions.len() {
            let opcode = self.instructions[self.rip];
            let operand = self.instructions[self.rip + 1];

            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => {
                    if self.jnz(operand) {
                        continue;
                    }
                }
                4 => self.bxc(operand),
                5 => {
                    outputs.push(self.out(operand));
                }
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => {
                    // Invalid Opcode
                }
            }

            self.rip += 2;
        }

        outputs
    }

    fn get_combo_operand(&self, op: u8) -> i64 {
        match op {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => op as i64,
        }
    }

    fn div(&self, operand: u8) -> i64 {
        let numerator = self.reg_a;
        let denominator = 1 << (self.get_combo_operand(operand));

        numerator / denominator
    }

    fn adv(&mut self, operand: u8) {
        self.reg_a = self.div(operand);
    }

    fn bdv(&mut self, operand: u8) {
        self.reg_b = self.div(operand);
    }

    fn cdv(&mut self, operand: u8) {
        self.reg_c = self.div(operand);
    }

    fn bxl(&mut self, operand: u8) {
        self.reg_b = self.reg_b ^ (operand as i64);
    }

    fn bst(&mut self, operand: u8) {
        self.reg_b = self.get_combo_operand(operand) % 8;
    }

    // Ruturn whether successful jump
    fn jnz(&mut self, operand: u8) -> bool {
        if self.reg_a != 0 {
            self.rip = operand as usize;
            return true;
        }

        false
    }

    fn bxc(&mut self, _operand: u8) {
        self.reg_b = self.reg_b ^ self.reg_c;
    }

    fn out(&self, operand: u8) -> u8 {
        (self.get_combo_operand(operand) % 8) as u8
    }
}

/*
For each value in instruction, we can brute-force to find the associating 4 * 3 bits whose result matches the target value.

Instructions: [2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0]
2,4 => B = A % 8
1,1 => B = B ^ 1
7,5 => C = A / (1 << B) => C = A >> B
4,6 => B = B ^ C

0,3 => A = A / 8

1,4 => B = B ^ 4
5,5 => print(B % 8)

3,0 => If A > 0, Start Over
*/

// Returns minimum initial value of register A
fn find_valid_reg_a(targets: &Vec<u8>, depth: usize, reg_a: u64) -> Option<u64> {
    if depth >= 4 {
        let target = targets[depth - 4];
        let current_reg_a = reg_a >> (3 * (depth - 4));
        let mut reg_b = current_reg_a % 8;
        reg_b = reg_b ^ 1;
        assert!(reg_b < 8, "reg_b must less than 8");
        let reg_c = current_reg_a >> reg_b;
        reg_b = reg_b ^ reg_c;
        reg_b = reg_b ^ 4;
        reg_b = reg_b % 8;

        if target != reg_b as u8 {
            return None;
        }

        if depth - 4 + 1 >= targets.len() {
            if target == reg_b as u8 {
                // Check Validity
                let mut computer = Computer::new(reg_a as i64, 0, 0, targets.clone());
                let out = computer.execute();
                if out == targets.clone() {
                    // Valid Value
                    return Some(reg_a);
                }
            }
            // Not Valid
            return None;
        }
    }

    let mut min_reg_a: Option<u64> = None;
    for i in 0..8 {
        let new_reg_a = (i << (3 * depth)) + reg_a;
        let result = find_valid_reg_a(targets, depth + 1, new_reg_a);
        if result.is_some() {
            // There is a possible initial reg_a, select the minimum value
            if min_reg_a.is_none() {
                min_reg_a = Some(result.unwrap());
            } else {
                min_reg_a = Some(min_reg_a.unwrap().min(result.unwrap()));
            }
        }
    }

    min_reg_a
}

fn main() {
    let inst = vec![2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0];

    // Part 1
    let mut computer = Computer::new(28066687, 0, 0, inst.clone());
    let outputs: Vec<String> = computer
        .execute()
        .into_iter()
        .map(|e| e.to_string())
        .collect();
    println!("Output: {}", outputs.join(","));

    // Part 2
    let answer = find_valid_reg_a(&inst, 0, 0);

    // Validate Answer
    let mut computer = Computer::new(answer.unwrap() as i64, 0, 0, inst.clone());
    let out = computer.execute();
    println!("{:?}", out);
    assert_eq!(out, inst);

    println!("Minimum Possible Register A: {:?}", answer);
}
