mod day_1;
mod day_2;

pub use day_1::Day1;
pub use day_2::Day2;

pub trait Day {
    fn part_1(&self, input: &str) -> String;
    fn part_2(&self, input: &str) -> String;
}