use crate::day::Solution;

pub struct GiftShop {
    pub ranges: Vec<(i64, i64)>,
}

impl GiftShop {
    pub fn new(input: &'static str) -> Self {
        let trimmed = input.trim().replace('\n', "");
        Self {
            ranges: trimmed
                .split(',')
                .map(|l| {
                    let (a, b) = l.split_once('-').unwrap();
                    (a.parse().unwrap(), b.parse().unwrap())
                })
                .collect(),
        }
    }
}

fn create_groups(chars: Vec<char>, group_size: usize) -> Vec<String> {
    let mut groups = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let mut group = String::new();
        for j in 0..group_size {
            group.push(chars[i + j]);
        }
        groups.push(group);
        i += group_size;
    }
    groups
}

impl Solution<i64, i64> for GiftShop {
    fn part_one(&mut self) -> i64 {
        self.ranges.iter().fold(0, |acc, (a, b)| {
            let range = *a..=*b;
            let mut invalid_id_sum_for_this_range = 0;
            for i in range {
                if i.to_string().len() % 2 != 0 || i < 11 {
                    continue;
                }

                let string = i.to_string();
                let (x, y) = string.split_at(i.to_string().chars().count() / 2);

                if x == y {
                    invalid_id_sum_for_this_range += i;
                }
            }
            acc + invalid_id_sum_for_this_range
        })
    }
    fn part_two(&mut self) -> i64 {
        self.ranges.iter().fold(0, |acc, (a, b)| {
            let range = *a..=*b;
            let mut invalid_id_sum_for_this_range = 0;
            for i in range {
                if i < 11 {
                    continue;
                }

                let string = i.to_string();

                let first_char = string.chars().next().unwrap();
                if string.chars().all(|c| c == first_char) {
                    invalid_id_sum_for_this_range += i;
                    continue;
                }

                if string.len() % 2 == 0 {
                    let (x, y) = string.split_at(i.to_string().chars().count() / 2);
                    if x == y {
                        invalid_id_sum_for_this_range += i;
                        continue;
                    }

                    if string.len() < 6 {
                        continue;
                    }

                    let pairs = create_groups(string.chars().collect(), 2);
                    if pairs.iter().all(|p| *p == pairs[0]) {
                        invalid_id_sum_for_this_range += i;
                    }

                    continue;
                }

                if string.len() == 9 {
                    let triplets = create_groups(string.chars().collect(), 3);

                    if triplets.iter().all(|t| *t == triplets[0]) {
                        invalid_id_sum_for_this_range += i;
                    }
                }
            }
            acc + invalid_id_sum_for_this_range
        })
    }
}
