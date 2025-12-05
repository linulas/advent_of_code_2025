use std::ops::RangeInclusive;

use crate::day::Solution;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    fn to_range_inclusive(&self) -> RangeInclusive<u64> {
        self.start..=self.end
    }
    fn overlaps(&self, other: &Range) -> bool {
        let r = self.to_range_inclusive();
        r.contains(&other.start) || r.contains(&other.end)
    }
}

impl From<&str> for Range {
    fn from(line: &str) -> Self {
        let (a, b) = line
            .split_once("-")
            .expect("line should contain two values separated by a '-'");
        Range {
            start: a.parse::<u64>().expect("a should be a number"),
            end: b.parse::<u64>().expect("b should be a number"),
        }
    }
}

pub struct Cafeteria {
    pub ranges: Vec<Range>,
    pub ingredients: Vec<u64>,
}

impl Cafeteria {
    pub fn new(input: &'static str) -> Self {
        let (a, b) = input
            .split_once("\n\n")
            .expect("input should contain two lines");
        Self {
            ranges: a.lines().map(Range::from).collect(),
            ingredients: b
                .lines()
                .map(|l| l.parse::<u64>().expect("line should be a number"))
                .collect(),
        }
    }
}

impl Solution<usize, u64> for Cafeteria {
    fn part_one(&mut self) -> usize {
        self.ingredients
            .iter()
            .filter(|i| {
                self.ranges
                    .iter()
                    .any(|r| r.to_range_inclusive().contains(i))
            })
            .count()
    }
    fn part_two(&mut self) -> u64 {
        self.ranges.sort();
        let mut intersected_ranges: Vec<Range> = Vec::new();

        self.ranges.iter().for_each(|r| {
            if let Some(i) = intersected_ranges.iter_mut().find(|ir| ir.overlaps(r)) {
                *i = Range {
                    start: std::cmp::min(i.start, r.start),
                    end: std::cmp::max(i.end, r.end),
                };
            } else {
                intersected_ranges.push(r.clone());
            }
        });

        intersected_ranges
            .iter()
            .fold(0, |acc, r| acc + (r.end - r.start + 1))
    }
}
