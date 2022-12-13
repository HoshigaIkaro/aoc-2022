use std::collections::{HashMap, HashSet, VecDeque};

use super::Day;

type Point = (usize, usize);

pub struct Day12;

impl Day for Day12 {
    fn part_1(&self, input: &str) -> String {
        let board: Vec<usize> = input
            .lines()
            .flat_map(|line| line.chars().map(char_to_height))
            .collect();
        let cols = input.find('\n').unwrap();
        let rows = board.len() / cols;

        let start_index = board.iter().position(|h| *h == 0).unwrap();
        let start_point = (start_index % cols, start_index / cols);

        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = std::iter::once(start_point).collect();
        let mut dist: HashMap<Point, usize> = std::iter::once((start_point, 0)).collect();

        while let Some(point @ (x, y)) = queue.pop_front() {
            if !visited.insert(point) {
                continue;
            }

            let current_height = board[y * cols + x];
            if current_height == 27 {
                return dist[&point].to_string();
            }

            for new_point @ (x_n, y_n) in get_new_positions(point, rows, cols) {
                let new_height = board[y_n * cols + x_n];
                if new_height <= current_height + 1 {
                    let current_dist = dist[&point];
                    let new_dist = dist.entry(new_point).or_insert(usize::MAX);
                    if current_dist + 1 < *new_dist {
                        dist.insert(new_point, current_dist + 1);
                        queue.push_back(new_point);
                    }
                }
            }
        }

        unreachable!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

fn char_to_height(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'S' => 0,
        'E' => 27,
        _ => unreachable!(),
    }
}

fn get_new_positions((col, row): Point, rows: usize, cols: usize) -> Vec<Point> {
    let mut positions = Vec::new();
    if row > 0 {
        positions.push((col, row - 1));
    }
    if row < rows - 1 {
        positions.push((col, row + 1));
    }
    if col > 0 {
        positions.push((col - 1, row));
    }
    if col < cols - 1 {
        positions.push((col + 1, row));
    }
    positions
}
