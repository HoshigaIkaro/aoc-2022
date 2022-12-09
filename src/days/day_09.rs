use std::{
    collections::{HashSet, BTreeMap},
    iter,
};

use super::Day;

pub struct Day09;

impl Day for Day09 {
    fn part_1(&self, input: &str) -> String {
        let mut positions: HashSet<(isize, isize)> = HashSet::from_iter(vec![(0, 0)]);
        let mut head: (isize, isize) = (0, 0);
        let mut tail: (isize, isize) = (0, 0);
        for step in input.lines() {
            let (dir, times) = step.split_once(char::is_whitespace).unwrap();
            let times = times.parse::<isize>().unwrap();
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
            while move_knot(&head, &mut tail) {
                positions.insert(tail);
            }
        }
        positions.len().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut knots: BTreeMap<usize, (isize, isize)> =
            BTreeMap::from_iter((0..=9).zip(iter::repeat((0, 0))));
        let mut positions: HashSet<(isize, isize)> = HashSet::from_iter(vec![(0, 0)]);
        for step in input.lines() {
            let (dir, times) = step.split_once(char::is_whitespace).unwrap();
            match dir {
                "R" => {
                    for _ in 0..times.parse().unwrap() {
                        let head = knots.get_mut(&0).unwrap();
                        head.0 += 1;

                        if update_knots(&mut knots) {
                            positions.insert(knots[&9]);
                        }
                    }
                }
                "L" => {
                    for _ in 0..times.parse().unwrap() {
                        let head = knots.get_mut(&0).unwrap();
                        head.0 -= 1;

                        if update_knots(&mut knots) {
                            positions.insert(knots[&9]);
                        }
                    }
                }
                "U" => {
                    for _ in 0..times.parse().unwrap() {
                        let head = knots.get_mut(&0).unwrap();
                        head.1 += 1;

                        if update_knots(&mut knots) {
                            positions.insert(knots[&9]);
                        }
                    }
                }
                "D" => {
                    for _ in 0..times.parse().unwrap() {
                        let head = knots.get_mut(&0).unwrap();
                        head.1 -= 1;

                        if update_knots(&mut knots) {
                            positions.insert(knots[&9]);
                        }
                    }
                }
                _ => (),
            };
        }
        positions.len().to_string()
    }
}

fn move_knot(head: &(isize, isize), tail: &mut (isize, isize)) -> bool {
    let v_d = head.1 - tail.1;
    let h_d = head.0 - tail.0;

    if h_d.abs() > 1 || v_d.abs() > 1 {
        tail.0 += h_d.signum();
        tail.1 += v_d.signum();
        true
    } else if h_d.abs() > 1 {
        tail.0 += h_d.signum();
        true
    } else if v_d.abs() > 1 {
        tail.1 += v_d.signum();
        true
    } else {
        false
    }
}

fn update_knots(knots: &mut BTreeMap<usize, (isize, isize)>) -> bool {
    let mut moved = false;
    for n in 1..=9 {
        let head = knots.get(&(n - 1)).unwrap();
        let mut tail = *knots.get(&n).unwrap();
        moved = move_knot(head, &mut tail);
        if moved {
            knots.insert(n, tail);
        }
    }
    moved
}
