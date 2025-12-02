use crate::day::Solution;

const START: i32 = 50;
const DIALS: i32 = 100;

pub struct SecretEntrance {
    pub rotations: Vec<Direction>,
}

impl SecretEntrance {
    pub fn new(input: &'static str) -> Self {
        Self {
            rotations: input.lines().map(Direction::from).collect(),
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Left(i32),
    Right(i32),
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        let (direction, distance) = value.split_at(1);
        match direction {
            "L" => Direction::Left(distance.parse().unwrap()),
            "R" => Direction::Right(distance.parse().unwrap()),
            _ => panic!(),
        }
    }
}

impl Solution<i32, usize> for SecretEntrance {
    fn part_one(&mut self) -> i32 {
        let mut count = 0;
        let mut pos = START;

        for rotoation in self.rotations.iter() {
            match rotoation {
                Direction::Left(x) => pos -= x % DIALS,
                Direction::Right(x) => pos += x % DIALS,
            }

            if pos < 0 {
                pos += DIALS;
            }

            if pos >= DIALS {
                pos -= DIALS;
            }

            if pos == 0 {
                count += 1;
            }
        }
        count
    }

    fn part_two(&mut self) -> usize {
        let mut count = 0;
        let mut pos = START;

        fn solve(rotation: i32, pos: &mut i32, result: &mut usize) {
            let new_position = *pos + rotation;

            *result += (new_position / DIALS).unsigned_abs() as usize;

            if new_position <= 0 && *pos != 0 {
                *result += 1;
            }

            *pos = (new_position % DIALS + DIALS) % DIALS
        }

        for rotoation in self.rotations.iter() {
            match rotoation {
                Direction::Left(x) => solve(-x, &mut pos, &mut count),
                Direction::Right(x) => solve(*x, &mut pos, &mut count),
            };
        }

        count
    }
}
