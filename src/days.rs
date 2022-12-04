mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub use day_01::Day01;
pub use day_02::Day02;
pub use day_03::Day03;
pub use day_04::Day04;

pub trait Day {
    fn part_1(&self, input: &str) -> String;
    fn part_2(&self, input: &str) -> String;
}
