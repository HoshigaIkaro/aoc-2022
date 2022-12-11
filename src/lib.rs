use utils::load_input;

pub mod days;
mod utils;

pub fn run_day(day: u8) {
    println!("Day: {day}");

    let input = load_input(day);
    let day: Box<dyn days::Day> = match day {
        1 => Box::new(days::Day01),
        2 => Box::new(days::Day02),
        3 => Box::new(days::Day03),
        4 => Box::new(days::Day04),
        5 => Box::new(days::Day05),
        6 => Box::new(days::Day06),
        7 => Box::new(days::Day07),
        8 => Box::new(days::Day08),
        9 => Box::new(days::Day09),
        10 => Box::new(days::Day10),
        _ => unreachable!(),
    };

    let one = day.part_1(&input);
    println!("- Part 1:");
    println!("{one}\n");

    let two = day.part_2(&input);
    println!("- Part 2:");
    println!("{two}");

    println!("------------------");
}
