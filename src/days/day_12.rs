use std::ops::Index;

use std::collections::{BinaryHeap, HashMap, HashSet};

type Point = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Path {
    position: Point,
    steps: usize,
}

impl Path {
    fn new(position: Point) -> Self {
        Self { position, steps: 0 }
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps) // min heap
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

use super::Day;

pub struct Day12;

impl Day for Day12 {
    fn part_1(&self, input: &str) -> String {
        let board: Vec<Vec<usize>> = input
            .lines()
            .map(|line| line.chars().map(char_to_height).collect())
            .collect();

        let start_index = board
            .iter()
            .flatten()
            .position(|c| *c == 'a' as usize - 1)
            .unwrap();
        let rows = board.len();
        let cols = board[0].len();
        dbg!(rows, cols);
        let start_point = (start_index % cols, start_index / cols);
        let mut dist: HashMap<Point, usize> = HashMap::new();
        dist.insert(start_point, 0);
        let mut queue: BinaryHeap<Path> = BinaryHeap::new();
        queue.push(Path::new(start_point));

        while let Some(Path { position, steps }) = queue.pop() {
            // dbg!(position);
            let current_height = board[position.1][position.0];
            if current_height == 'z' as usize + 1 {
                dbg!(steps);
            }

            if steps > dist[&position] {
                continue;
            }

            for new_position @ (x, y) in get_new_positions(position, rows, cols) {
                if board[y][x] > current_height + 1{
                    continue;
                }
                let next = Path {
                    steps: steps + 1,
                    position: new_position,
                };

                // dbg!('h');
                if next.steps < *dist.entry(new_position).or_insert(usize::MAX) {
                    queue.push(next);
                    dist.insert(new_position, next.steps);
                }
            }
        }
        // println!("{:?}", dist);
        unreachable!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

fn char_to_height(c: char) -> usize {
    match c {
        'a'..='z' => c as usize,
        'S' => 'a' as usize - 1,
        'E' => 'z' as usize + 1,
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
