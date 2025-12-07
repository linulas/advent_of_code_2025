use crate::day::print_day;
use solutions::day_01::SecretEntrance;
use solutions::day_02::GiftShop;
use solutions::day_03::EmergancyPower;
use solutions::day_04::PrintingDepartment;
use solutions::day_05::Cafeteria;
use solutions::day_06::MathHomework;
use solutions::day_07::TachyonManifolds;
use std::env;

#[cfg(test)]
mod test;

mod day;
mod solutions;

const ARGUMENT_ERROR: &str = "Please provide a day (number 1-24).";
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{ARGUMENT_ERROR}");
        return;
    }

    let number = match args[1].parse::<u8>() {
        Ok(num) => {
            if !(1..=25).contains(&num) {
                println!("{ARGUMENT_ERROR}");
                return;
            }
            num
        }
        Err(_) => {
            println!("{ARGUMENT_ERROR}");
            return;
        }
    };

    match number {
        1 => print_day(1, SecretEntrance::new(include_str!("input/01.txt"))),
        2 => print_day(2, GiftShop::new(include_str!("input/02.txt"))),
        3 => print_day(3, EmergancyPower::new(include_str!("input/03.txt"))),
        4 => print_day(4, PrintingDepartment::new(include_str!("input/04.txt"))),
        5 => print_day(5, Cafeteria::new(include_str!("input/05.txt"))),
        6 => print_day(6, MathHomework::new(include_str!("input/06.txt"))),
        7 => print_day(7, TachyonManifolds::new(include_str!("input/07.txt"))),
        8 => todo!(),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        12 => todo!(),
        13 => todo!(),
        14 => todo!(),
        15 => todo!(),
        16 => todo!(),
        17 => todo!(),
        18 => todo!(),
        19 => todo!(),
        20 => todo!(),
        21 => todo!(),
        22 => todo!(),
        23 => todo!(),
        24 => todo!(),
        25 => todo!(),
        _ => unreachable!(),
    }
}
