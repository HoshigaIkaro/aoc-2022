use super::Day;

pub struct Day02;

#[allow(clippy::identity_op)]
impl Day for Day02 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|s| match s {
                // Opponent: Rock
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                // Opponent: Paper
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                // Opponent: Scissors
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .lines()
            .map(|s| match s {
                // Opponent: Rock
                "A X" => 3 + 0,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,
                // Opponent: Paper
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                // Opponent: Scissors
                "C X" => 2 + 0,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }
}
