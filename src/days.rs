mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

pub use day_01::Day01;
pub use day_02::Day02;
pub use day_03::Day03;
pub use day_04::Day04;
pub use day_05::Day05;
pub use day_06::Day06;
pub use day_07::Day07;
pub use day_08::Day08;
pub use day_09::Day09;
pub use day_10::Day10;
pub use day_11::Day11;
pub use day_12::Day12;
pub use day_13::Day13;
pub use day_14::Day14;
pub use day_15::Day15;
pub use day_16::Day16;
pub use day_17::Day17;

pub trait Day {
    fn part_1(&self, input: &str) -> String;
    fn part_2(&self, input: &str) -> String;
}
