use crate::day::Solution;

pub struct EmergancyPower {
    pub battery_banks: Vec<Vec<u64>>,
}

impl EmergancyPower {
    pub fn new(input: &'static str) -> Self {
        Self {
            battery_banks: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).expect("Should be a digit") as u64)
                        .collect()
                })
                .collect(),
        }
    }
}

fn get_joltage(bank: &[u64], battery_count: usize) -> u64 {
    let mut batteries = bank[bank.len() - battery_count..].to_vec();

    (0..(bank.len() - battery_count)).rev().for_each(|i| {
        let mut battery = bank[i];

        #[allow(clippy::needless_range_loop)]
        for j in 0..batteries.len() {
            if battery >= batteries[j] {
                std::mem::swap(&mut batteries[j], &mut battery);
                continue;
            }
            break;
        }
    });

    let joltage: String = batteries.iter().map(|b| b.to_string()).collect();
    joltage.parse::<u64>().expect("Should be a number")
}

impl Solution<u64, u64> for EmergancyPower {
    fn part_one(&mut self) -> u64 {
        self.battery_banks
            .iter()
            .fold(0, |acc, bank| acc + get_joltage(bank, 2))
    }
    fn part_two(&mut self) -> u64 {
        self.battery_banks
            .iter()
            .fold(0, |acc, bank| acc + get_joltage(bank, 12))
    }
}
