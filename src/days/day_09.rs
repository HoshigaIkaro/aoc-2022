use std::collections::HashSet;

use super::Day;

/// Position
type Position = (isize, isize);
type Positions = HashSet<Position>;

pub struct Day09;

impl Day for Day09 {
    fn part_1(&self, input: &str) -> String {
        let mut positions: Positions = HashSet::from_iter(vec![(0, 0)]);
        let mut knots = [(0, 0); 2];
        for step in input.lines() {
            let (dir, times) = step.split_once(char::is_whitespace).unwrap();
            let times = times.parse::<isize>().unwrap();
            let head = &mut knots[0];
            match dir {
                "R" => {
                    head.0 += times;
                }
                "L" => {
                    head.0 -= times;
                }
                "U" => {
                    head.1 += times;
                }
                "D" => {
                    head.1 -= times;
                }
                _ => unreachable!(),
            };
            while update_knots::<2>(&mut knots) {
                positions.insert(knots[1]);
            }
        }
        positions.len().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut knots = [(0, 0); 10];
        let mut positions: Positions = HashSet::from_iter(vec![(0, 0)]);
        for step in input.lines() {
            let (dir, times) = step.split_once(char::is_whitespace).unwrap();
            match dir {
                "R" => {
                    for _ in 0..times.parse().unwrap() {
                        let head = &mut knots[0];
                        head.0 += 1;

                        if update_knots::<10>(&mut knots) {
                            positions.insert(knots[9]);
                        }
                    }
                }
                "L" => {
                    for _ in 0..times.parse().unwrap() {
                        let head = &mut knots[0];
                        head.0 -= 1;

                        if update_knots::<10>(&mut knots) {
                            positions.insert(knots[9]);
                        }
                    }
                }
                "U" => {
                    for _ in 0..times.parse().unwrap() {
                        let head = &mut knots[0];
                        head.1 += 1;

                        if update_knots::<10>(&mut knots) {
                            positions.insert(knots[9]);
                        }
                    }
                }
                "D" => {
                    for _ in 0..times.parse().unwrap() {
                        let head = &mut knots[0];
                        head.1 -= 1;

                        if update_knots::<10>(&mut knots) {
                            positions.insert(knots[9]);
                        }
                    }
                }
                _ => (),
            };
        }
        positions.len().to_string()
    }
}

fn move_knot(head: Position, tail: &mut Position) -> bool {
    let v_d = head.1 - tail.1;
    let h_d = head.0 - tail.0;

    if h_d.abs() > 1 || v_d.abs() > 1 {
        tail.0 += h_d.signum();
        tail.1 += v_d.signum();
        true
    } else {
        false
    }
}

fn update_knots<const K: usize>(knots: &mut [Position; K]) -> bool {
    let mut moved = false;
    for n in 1..K {
        let head = knots[n - 1];
        let tail = &mut knots[n];
        moved = move_knot(head, tail);
        if moved {
            knots[n] = *tail;
        }
    }
    moved
}
