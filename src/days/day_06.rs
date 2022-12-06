use std::ops::Add;

use super::Day;

pub struct Day06;

impl Day for Day06 {
    fn part_1(&self, input: &str) -> String {
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(4)
            .enumerate()
            .find(|(_i, window)| {
                window
                    .iter()
                    .enumerate()
                    .all(|(i, c)| window.iter().skip(i + 1).find(|d| c == *d).is_none())
            })
            .unwrap()
            .0
            .add(4)
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(14)
            .enumerate()
            .find(|(_i, window)| {
                window
                    .iter()
                    .enumerate()
                    .all(|(i, c)| window.iter().skip(i + 1).find(|d| c == *d).is_none())
            })
            .unwrap()
            .0
            .add(14)
            .to_string()
    }
}
