use utils::load_input;

pub mod days;
mod utils;

pub fn run_day(day: u8) {
    let input = load_input(day);
    let day: Box<dyn days::Day> = match day {
        1 => Box::new(days::Day01),
        2 => Box::new(days::Day02),
        3 => Box::new(days::Day03),
        4 => Box::new(days::Day04),
        _ => unreachable!(),
    };

    let one = day.part_1(&input);
    println!("{one}");
    let two = day.part_2(&input);
    println!("{two}");
}
