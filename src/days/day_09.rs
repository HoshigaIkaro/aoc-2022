use std::collections::HashSet;

use super::Day;

type Positions = HashSet<Point>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Point {
    fn sign_delta(&self) -> Self {
        Self(self.0.signum(), self.1.signum())
    }
}

pub struct Day09;

impl Day for Day09 {
    fn part_1(&self, input: &str) -> String {
        let mut positions: HashSet<Point> = HashSet::from_iter(std::iter::once(Point(0, 0)));
        let mut knots = [Point(0, 0); 2];
        for line in input.lines() {
            let (dir, times) = line.split_once(' ').unwrap();
            let times = times.parse::<isize>().unwrap();
            let delta = match dir {
                "R" => Point(times, 0),
                "L" => Point(-times, 0),
                "U" => Point(0, times),
                "D" => Point(0, -times),
                _ => unreachable!(),
            };
            knots[0] += delta;
            knots[1] = move_knot(knots[0], knots[1]);
        }
        dbg!(knots[1]);
        todo!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

fn move_knot(head: Point, mut tail: Point) -> Point {
    let delta @ Point(h_d, v_d) = head - tail;

    if (h_d.abs() == 1 && v_d.abs() > 1) || (h_d.abs() > 1 && v_d.abs() == 1) {
        dbg!(delta.sign_delta(), tail);
        tail += delta.sign_delta();
    }
    tail
}

#[cfg(test)]
mod day_09_tests {
    use super::*;

    #[test]
    fn move_knot_works() {
        
    }
}