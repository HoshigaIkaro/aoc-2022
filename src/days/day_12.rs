use std::collections::{HashMap, HashSet, VecDeque};

use super::Day;

type Point = (usize, usize);

pub struct Day12;

impl Day for Day12 {
    fn part_1(&self, input: &str) -> String {
        let chars: Vec<char> = input.lines().flat_map(str::chars).collect();

        let cols = input.find('\n').unwrap();
        let rows = chars.len() / cols;

        let start_index = chars.iter().position(|c| *c == 'S').unwrap();
        let start_point = (start_index % cols, start_index / cols);

        let end_index = chars.iter().position(|c| *c == 'E').unwrap();
        let end_point = (end_index % cols, end_index / cols);

        let board: Vec<usize> = chars.into_iter().map(char_to_height).collect();

        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = std::iter::once(start_point).collect();
        let mut dist: HashMap<Point, usize> = std::iter::once((start_point, 0)).collect();

        while let Some(point @ (x, y)) = queue.pop_front() {
            if !visited.insert(point) {
                continue;
            }

            if point == end_point {
                break;
            }

            let current_height = board[y * cols + x];
            for new_point @ (x_n, y_n) in get_new_positions(point, rows, cols) {
                let new_height = board[y_n * cols + x_n];
                if new_height <= current_height + 1 {
                    let current_dist = dist[&point];

                    let new_dist = dist.get(&new_point).unwrap_or(&usize::MAX);
                    if current_dist < *new_dist {
                        dist.insert(new_point, current_dist + 1);
                        queue.push_back(new_point);
                    }
                }
            }
        }

        dist[&end_point].to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let chars: Vec<char> = input.lines().flat_map(str::chars).collect();

        let cols = input.find('\n').unwrap();
        let rows = chars.len() / cols;

        let start_index = chars.iter().position(|c| *c == 'E').unwrap();
        let start_point = (start_index % cols, start_index / cols);

        let board: Vec<usize> = chars.into_iter().map(char_to_height_reverse).collect();

        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = std::iter::once(start_point).collect();
        let mut dist: HashMap<Point, usize> = std::iter::once((start_point, 0)).collect();

        while let Some(point @ (x, y)) = queue.pop_front() {
            if !visited.insert(point) {
                continue;
            }

            let current_height = board[y * cols + x];
            if current_height == 25 {
                return dist[&point].to_string();
            }

            for new_point @ (x_n, y_n) in get_new_positions(point, rows, cols) {
                let new_height = board[y_n * cols + x_n];
                if new_height <= current_height + 1 {
                    let current_dist = dist[&point];

                    let new_dist = dist.get(&new_point).unwrap_or(&usize::MAX);
                    if current_dist < *new_dist {
                        dist.insert(new_point, current_dist + 1);
                        queue.push_back(new_point);
                    }
                }
            }
        }

        unreachable!()
    }
}

fn char_to_height(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize,
        'S' => 0,
        'E' => 25,
        _ => unreachable!(),
    }
}

fn char_to_height_reverse(c: char) -> usize {
    match c {
        'a'..='z' => 'z' as usize - c as usize,
        'S' => 25,
        'E' => 0,
        _ => unreachable!(),
    }
}

fn get_new_positions((col, row): Point, rows: usize, cols: usize) -> Vec<Point> {
    let mut positions = Vec::with_capacity(4);
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

#[cfg(test)]
mod day_12_tests {
    use super::*;

    #[test]
    fn test_new_positions() {
        let rows = 2;
        let cols = 2;
        let point = (0, 0);
        assert_eq!(get_new_positions(point, rows, cols), vec![(0, 1), (1, 0)]);
        let point = (1, 1);
        assert_eq!(get_new_positions(point, rows, cols), vec![(1, 0), (0, 1)]);
        let point = (0, 1);
        assert_eq!(get_new_positions(point, rows, cols), vec![(0, 0), (1, 1)]);
        let point = (1, 0);
        assert_eq!(get_new_positions(point, rows, cols), vec![(1, 1), (0, 0)]);
    }
}
