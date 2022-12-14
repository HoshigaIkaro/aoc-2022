use owo_colors::OwoColorize;
use utils::load_input;

pub mod days;
mod utils;

pub fn run_day(day: u8) {
    println!("{}", format!("Day: {day}").fg_rgb::<198, 208, 245>());

    let input = load_input(day);
    let day: Box<dyn days::Day> = match day {
        1 => {
            dbg!(days::day_01::run(&input));
            Box::new(days::Day01)
        }
        2 => {
            dbg!(days::day_02::run(&input));
            Box::new(days::Day02)
        }
        3 => {
            dbg!(days::day_03::run(&input));
            Box::new(days::Day03)
        }
        4 => {
            dbg!(days::day_04::run(&input));
            Box::new(days::Day04)
        }
        5 => {
            dbg!(days::day_05::run(&input));
            Box::new(days::Day05)
        }
        6 => Box::new(days::Day06),
        7 => Box::new(days::Day07),
        8 => Box::new(days::Day08),
        9 => Box::new(days::Day09),
        10 => Box::new(days::Day10),
        11 => Box::new(days::Day11),
        12 => Box::new(days::Day12),
        13 => Box::new(days::Day13),
        14 => Box::new(days::Day14),
        15 => Box::new(days::Day15),
        16 => Box::new(days::Day16),
        17 => Box::new(days::Day17),
        18 => Box::new(days::Day18),
        19 => Box::new(days::Day19),
        20 => Box::new(days::Day20),
        21 => Box::new(days::Day21),
        22 => Box::new(days::Day22),
        23 => Box::new(days::Day23),
        24 => Box::new(days::Day24),
        25 => Box::new(days::Day25),
        _ => unreachable!(),
    };

    let one = day.part_1(&input);
    println!("{}", "- Part 1:".fg_rgb::<181, 191, 226>());
    println!("{}\n", one.fg_rgb::<166, 209, 137>());

    let two = day.part_2(&input);
    println!("{}", "- Part 2:".fg_rgb::<181, 191, 226>());
    println!("{}", two.fg_rgb::<166, 209, 137>());

    println!("{}", "------------------".fg_rgb::<115, 121, 148>());
}
