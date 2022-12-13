use std::ops::Add;

use super::Day;

pub struct Day08;

impl Day for Day08 {
    fn part_1(&self, input: &str) -> String {
        let trees: Vec<usize> = parse(input);
        let cols = input.find('\n').unwrap();
        let rows = trees.len() / cols;
        (1..rows - 1)
            .map(|row| {
                (1..cols - 1)
                    .filter(|&col| {
                        let height = trees[row * cols + col];
                        (0..col).all(|i| trees[row * cols + i] < height)
                            || (col + 1..cols).all(|i| trees[row * cols + i] < height)
                            || (0..row).all(|r| trees[r * cols + col] < height)
                            || (row + 1..rows).all(|r| trees[r * cols + col] < height)
                    })
                    .count()
            })
            .sum::<usize>()
            .add(2 * (rows + cols) - 4) // edges without overlaps
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let trees: Vec<usize> = parse(input);
        let cols = input.find('\n').unwrap();
        let rows = trees.len() / cols;
        (1..rows - 1)
            .map(|row| {
                (1..cols - 1)
                    .map(|col| {
                        let height = trees[row * cols + col];
                        let left = (0..col)
                            .enumerate()
                            .rev()
                            .find(|(_, c)| trees[row * cols + *c] >= height)
                            .map_or(col, |(index, _)| col - index);
                        let up = (0..row)
                            .enumerate()
                            .rev()
                            .find(|(_, r)| trees[*r * cols + col] >= height)
                            .map_or(row, |(index, _)| row - index);
                        let right = (col + 1..cols)
                            .enumerate()
                            .find(|(_, c)| trees[row * cols + *c] >= height)
                            .map_or(cols - col - 1, |(index, _)| index + 1);
                        let down = (row + 1..rows)
                            .enumerate()
                            .find(|(_i, r)| trees[*r * cols + col] >= height)
                            .map_or(rows - row - 1, |(index, _)| index + 1);
                        left * up * right * down
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
            .to_string()
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect()
}
