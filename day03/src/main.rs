#[derive(Debug, Clone)]
struct BatteryBank {
    voltages: Vec<usize>,
}

impl BatteryBank {
    fn _highest_voltage_naive(&self) -> usize {
        let mut max_voltage = 0;
        for (index, val) in self.voltages.iter().enumerate() {
            for (_index_right, val_right) in self.voltages[index + 1..].iter().enumerate().rev() {
                let voltage = val * 10 + val_right;
                if voltage > max_voltage {
                    max_voltage = voltage;
                }
            }
        }
        max_voltage
    }

    fn max_voltage_for_bank(&self, n: usize) -> usize {
        if n == 0 || n > self.voltages.len() {
            return 0;
        }

        let mut result: usize = 0;
        let mut start = 0;
        let mut remaining_to_pick = n;

        while remaining_to_pick > 0 {
            let end = self.voltages.len() - remaining_to_pick;

            let mut best_digit = 0;
            let mut best_index = start;

            for i in start..=end {
                let d = self.voltages[i];
                if d > best_digit {
                    best_digit = d;
                    best_index = i;
                }
            }

            result = result * 10 + best_digit;

            start = best_index + 1;
            remaining_to_pick -= 1;
        }

        result
    }
}

fn main() {
    let filname = "input.txt";
    let content = std::fs::read_to_string(filname).expect("Failed to read input file");
    let banks: Vec<BatteryBank> = content
        .split("\n")
        .map(|block| {
            let voltages = block
                .chars()
                .map(|c| {
                    c.to_string()
                        .parse::<usize>()
                        .expect("failed to parse character")
                })
                .collect();
            BatteryBank { voltages }
        })
        .collect();

    // let part01_orig: usize = banks.iter().map(|bank| bank.highest_voltage_naive()).sum();
    let part01: usize = banks.iter().map(|bank| bank.max_voltage_for_bank(2)).sum();
    let part02: usize = banks.iter().map(|bank| bank.max_voltage_for_bank(12)).sum();
    // println!("Part 01: {} *", part01_orig);
    println!("Part 01: {}", part01);
    println!("Part 02: {}", part02);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_highest_voltage() {
        let bank = BatteryBank {
            voltages: vec![8, 8, 8, 9, 2],
        };
        assert_eq!(bank.clone().max_voltage_for_bank(2), 92);
    }
}
