use super::Day;

pub struct Day04;

impl Day for Day04 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();
                (to_pair(left), to_pair(right))
            })
            .filter(|(left, right)| {
                (left.0 >= right.0 && left.1 <= right.1) || (right.0 >= left.0 && right.1 <= left.1)
            })
            .count()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();
                (to_pair(left), to_pair(right))
            })
            .filter(|(left, right)| left.0 <= right.1 && right.0 <= left.1)
            .count()
            .to_string()
    }
}

fn to_pair(input: &str) -> (usize, usize) {
    let (left, right) = input.split_once('-').unwrap();
    let left: usize = lexical::parse(left).unwrap();
    let right: usize = lexical::parse(right).unwrap();
    (left, right)
}
