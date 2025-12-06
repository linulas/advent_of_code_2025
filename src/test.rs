use crate::day::Solution;
use crate::solutions::day_01::SecretEntrance;
use crate::solutions::day_02::GiftShop;
use crate::solutions::day_03::EmergancyPower;
use crate::solutions::day_04::PrintingDepartment;
use crate::solutions::day_05::Cafeteria;
use crate::solutions::day_06::MathHomework;

// NOTE: do not remove the '// day x::' it will break importing the day with the cli.

#[test]
fn day_01() {
    let mut solution = SecretEntrance::new(include_str!("input/01_test.txt"));
    println!("rotations: {:?}", solution.rotations);
    assert_eq!(solution.rotations.len(), 10);
    assert_eq!(solution.part_one(), 3);
    assert_eq!(solution.part_two(), 6);
}

#[test]
fn day_02() {
    let mut solution = GiftShop::new(include_str!("input/02_test.txt"));
    println!("ranges: {:?}\n", solution.ranges);
    assert_eq!(solution.ranges.len(), 11);
    assert_eq!(solution.part_one(), 1227775554);
    assert_eq!(solution.part_two(), 4174379265);
}

#[test]
fn day_03() {
    let mut solution = EmergancyPower::new(include_str!("input/03_test.txt"));
    println!("battery banks: {:?}\n", solution.battery_banks);
    assert_eq!(solution.battery_banks.len(), 4);
    assert_eq!(solution.battery_banks[0].len(), 15);
    assert_eq!(solution.part_one(), 357);
    assert_eq!(solution.part_two(), 3121910778619);
}

#[test]
fn day_04() {
    let mut solution = PrintingDepartment::new(include_str!("input/04_test.txt"));
    println!("grid: {:?}\n", solution.grid);
    assert_eq!(solution.grid.len(), 100);
    assert_eq!(solution.count_adjecent((0, 0)), 2);
    assert_eq!(solution.count_adjecent((1, 2)), 7);
    assert_eq!(solution.part_one(), 13);
    assert_eq!(solution.part_two(), 43);
}

#[test]
fn day_05() {
    let mut solution = Cafeteria::new(include_str!("input/05_test.txt"));
    println!("ranges: {:?}\n", solution.ranges);
    println!("ingredients: {:?}\n", solution.ingredients);
    assert_eq!(solution.ranges.len(), 4);
    assert_eq!(solution.ingredients.len(), 6);
    assert_eq!(solution.part_one(), 3);
    assert_eq!(solution.part_two(), 14);
}

#[test]
fn day_06() {
    let mut solution = MathHomework::new(include_str!("input/06_test.txt"));
    println!("problems: {:?}\n", solution.problems);
    assert_eq!(solution.part_one(), 4277556);
    assert_eq!(solution.part_two(), 3263827);
}

#[test]
fn day_07() {
    todo!(); // day 7::
}

#[test]
fn day_08() {
    todo!(); // day 8::
}

#[test]
fn day_09() {
    todo!(); // day 9::
}

#[test]
fn day_10() {
    todo!(); // day 10::
}

#[test]
fn day_11() {
    todo!(); // day 11::
}

#[test]
fn day_12() {
    todo!(); // day 12::
}

#[test]
fn day_13() {
    todo!(); // day 13::
}

#[test]
fn day_14() {
    todo!(); // day 14::
}

#[test]
fn day_15() {
    todo!(); // day 15::
}

#[test]
fn day_16() {
    todo!(); // day 16::
}

#[test]
fn day_17() {
    todo!(); // day 17::
}

#[test]
fn day_18() {
    todo!(); // day 18::
}

#[test]
fn day_19() {
    todo!(); // day 19::
}

#[test]
fn day_20() {
    todo!(); // day 20::
}

#[test]
fn day_21() {
    todo!(); // day 21::
}

#[test]
fn day_22() {
    todo!(); // day 22::
}

#[test]
fn day_23() {
    todo!(); // day 23::
}

#[test]
fn day_24() {
    todo!(); // day 24::
}

#[test]
fn day_25() {
    todo!(); // day 25::
}
