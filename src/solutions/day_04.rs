use std::collections::HashMap;

use crate::day::Solution;

type Coord = (i32, i32);

pub struct PrintingDepartment {
    pub grid: HashMap<Coord, char>,
}

impl PrintingDepartment {
    pub fn new(input: &'static str) -> Self {
        Self {
            grid: input
                .lines()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (y, line)| {
                    line.chars().enumerate().for_each(|(x, c)| {
                        acc.insert((x as i32, y as i32), c);
                    });
                    acc
                }),
        }
    }
    pub fn count_adjecent(&self, (x, y): Coord) -> i32 {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(c) = self.grid.get(&(x + dx, y + dy)) {
                    if *c == '@' {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

impl Solution<i32, i32> for PrintingDepartment {
    fn part_one(&mut self) -> i32 {
        self.grid.iter().fold(0, |acc, (coord, c)| {
            if *c == '@' && self.count_adjecent(*coord) < 4 {
                acc + 1
            } else {
                acc
            }
        })
    }
    fn part_two(&mut self) -> i32 {
        let mut removed_rolls = 0;

        loop {
            let to_be_removed: Vec<Coord> =
                self.grid.iter().fold(Vec::new(), |mut acc, (coord, c)| {
                    if *c == '@' && self.count_adjecent(*coord) < 4 {
                        acc.push(*coord);
                    }

                    acc
                });

            if to_be_removed.is_empty() {
                break;
            }

            to_be_removed.iter().for_each(|coord| {
                if let Some(c) = self.grid.get_mut(coord) {
                    *c = 'x';
                }
                removed_rolls += 1;
            });
        }

        removed_rolls
    }
}
