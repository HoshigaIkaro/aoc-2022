use std::ops::Add;

use super::Day;

pub struct Day06;

impl Day for Day06 {
    fn part_1(&self, input: &str) -> String {
        find::<4>(input)
    }

    fn part_2(&self, input: &str) -> String {
        find::<14>(input)
    }
}

#[allow(clippy::search_is_some)]
fn find<const W: usize>(input: &str) -> String {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(W)
        .enumerate()
        .find(|(_i, window)| {
            window
                .iter()
                .enumerate()
                .all(|(i, c)| window.iter().skip(i + 1).find(|d| c == *d).is_none())
        })
        .unwrap()
        .0
        .add(W)
        .to_string()
}
