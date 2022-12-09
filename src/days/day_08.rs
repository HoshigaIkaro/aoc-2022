use std::ops::Add;

use super::Day;

pub struct Day08;

impl Day for Day08 {
    fn part_1(&self, input: &str) -> String {
        let grid: Vec<Vec<usize>> = parse(input);
        let rows = grid.len();
        let cols = grid[0].len();
        (1..rows - 1)
            .map(|row| {
                (1..cols - 1)
                    .filter(|&col| {
                        let height = grid[row][col];
                        (0..col).all(|i| grid[row][i] < height)
                            || (col + 1..cols).all(|i| grid[row][i] < height)
                            || (0..row).all(|r| grid[r][col] < height)
                            || (row + 1..rows).all(|r| grid[r][col] < height)
                    })
                    .count()
            })
            .sum::<usize>()
            .add(2 * (rows + cols) - 4) // edges without overlaps
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let grid = parse(input);
        let rows = grid.len();
        let cols = grid[0].len();
        (1..rows - 1)
            .map(|row| {
                (1..cols - 1)
                    .map(|col| {
                        let height = grid[row][col];
                        let left = (0..col)
                            .enumerate()
                            .rev()
                            .find(|(_, c)| grid[row][*c] >= height)
                            .map_or(col, |(index, _)| col - index);
                        let up = (0..row)
                            .enumerate()
                            .rev()
                            .find(|(_, r)| grid[*r][col] >= height)
                            .map_or(row, |(index, _)| row - index);
                        let right = (col + 1..cols)
                            .enumerate()
                            .find(|(_, c)| grid[row][*c] >= height)
                            .map_or(cols - col - 1, |(index, _)| index + 1);
                        let down = (row + 1..rows)
                            .enumerate()
                            .find(|(_i, r)| grid[*r][col] >= height)
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

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}
