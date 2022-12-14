use std::collections::HashSet;

use super::Day;

type Point = (isize, isize);

pub struct Day14;

impl Day for Day14 {
    fn part_1(&self, input: &str) -> String {
        let mut occupied: HashSet<Point> = HashSet::new();
        let mut depth = 0;
        for line in input.lines() {
            let mut lines = line.split(" -> ").map(|p| {
                let (x, y) = p.split_once(',').unwrap();
                (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
            });
            let mut current = lines.next().unwrap();
            while let Some(new) = lines.next() {
                depth = depth.max(new.1);
                occupied.insert(current);
                while current != new {
                    current.0 += (new.0 - current.0).signum();
                    current.1 += (new.1 - current.1).signum();
                    occupied.insert(current);
                }
            }
        }

        let mut count = 0;
        'outer: loop {
            let mut current = (500, 0);

            loop {
                // println!("{current:?}");
                if current.1 >= depth {
                    break 'outer;
                }

                // straight down;
                let new = (current.0, current.1 + 1);
                if !occupied.contains(&new) {
                    current = new;
                    continue;
                }

                // lower left
                let new = (current.0 - 1, current.1 + 1);
                if !occupied.contains(&new) {
                    current = new;
                    continue;
                }

                // lower right
                let new = (current.0 + 1, current.1 + 1);
                if !occupied.contains(&new) {
                    current = new;
                    continue;
                }

                // no moves left
                occupied.insert(current);
                break;
            }

            count += 1;
        }
        count.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut occupied: HashSet<Point> = HashSet::new();
        let mut depth = 0;
        for line in input.lines() {
            let mut lines = line.split(" -> ").map(|p| {
                let (x, y) = p.split_once(',').unwrap();
                (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
            });
            let mut current = lines.next().unwrap();
            while let Some(new) = lines.next() {
                depth = depth.max(new.1);
                occupied.insert(current);
                while current != new {
                    current.0 += (new.0 - current.0).signum();
                    current.1 += (new.1 - current.1).signum();
                    occupied.insert(current);
                }
            }
        }
        let depth = depth + 2;
        let mut count = 0;
        'outer: loop {
            let mut current = (500, 0);

            loop {
                if current.1 == depth - 1 {
                    occupied.insert(current);
                    break;
                }

                // straight down;
                let new = (current.0, current.1 + 1);
                if !occupied.contains(&new) {
                    current = new;
                    continue;
                }

                // lower left
                let new = (current.0 - 1, current.1 + 1);
                if !occupied.contains(&new) {
                    current = new;
                    continue;
                }

                // lower right
                let new = (current.0 + 1, current.1 + 1);
                if !occupied.contains(&new) {
                    current = new;
                    continue;
                }
                // no moves left
                if current == (500, 0) {
                    count += 1;
                    break 'outer;
                }
                occupied.insert(current);
                break;
            }
            count += 1;
        }
        count.to_string()
    }
}
