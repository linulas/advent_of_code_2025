use std::collections::{HashMap, HashSet};

use crate::day::Solution;

pub struct TachyonManifolds {
    pub input: &'static str,
}

impl TachyonManifolds {
    pub fn new(input: &'static str) -> Self {
        Self { input }
    }
    pub fn start_position(&self) -> usize {
        if let Some(line) = self.input.lines().next() {
            line.chars()
                .position(|c| c == 'S')
                .expect("There should be a start char 'S' in the first line")
        } else {
            panic!("There should be at least one line in the input");
        }
    }
}

fn get_intersections(
    line: &str,
    beam_positions: &mut HashMap<usize, usize>,
) -> HashMap<usize, usize> {
    let splitter_positions: HashSet<usize> = HashSet::from_iter(
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '^')
            .map(|(pos, _)| (pos)),
    );

    let mut intersections: HashMap<usize, usize> = HashMap::new();

    for (key, value) in &mut *beam_positions {
        if splitter_positions.contains(key) {
            intersections.insert(*key, *value);
        }
    }

    beam_positions.retain(|key, _| !intersections.contains_key(key));

    intersections
}

impl Solution<usize, usize> for TachyonManifolds {
    fn part_one(&mut self) -> usize {
        let mut beam_positions: HashMap<usize, usize> = HashMap::new();
        beam_positions.insert(self.start_position(), 0);
        self.input.lines().skip(1).fold(0, |acc, line| {
            let intersections = get_intersections(line, &mut beam_positions);

            intersections.iter().for_each(|pos| {
                beam_positions.entry(*pos.0 + 1).or_insert(0);
                beam_positions.entry(*pos.0 - 1).or_insert(0);
            });

            acc + intersections.len()
        })
    }
    fn part_two(&mut self) -> usize {
        let mut beam_positions: HashMap<usize, usize> = HashMap::new();
        beam_positions.insert(self.start_position(), 1);
        self.input.lines().skip(1).for_each(|line| {
            let intersections = get_intersections(line, &mut beam_positions);

            intersections.iter().for_each(|(pos, value)| {
                beam_positions
                    .entry(*pos + 1)
                    .and_modify(|e| *e += *value)
                    .or_insert(*value);

                beam_positions
                    .entry(*pos - 1)
                    .and_modify(|e| *e += *value)
                    .or_insert(*value);
            });
        });
        beam_positions.values().sum()
    }
}
