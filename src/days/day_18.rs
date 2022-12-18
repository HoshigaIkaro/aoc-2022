use std::collections::{HashSet, VecDeque};

use super::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Ord)]
struct Point {
    /// left to right: - -> +
    x: isize,
    /// down to up: - -> +
    y: isize,
    /// back to front: - -> +
    z: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Cube(Point);

impl Cube {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self(Point { x, y, z })
    }
    fn front(&self) -> Point {
        Point {
            z: self.0.z + 1,
            ..self.0
        }
    }
    fn back(&self) -> Point {
        Point {
            z: self.0.z - 1,
            ..self.0
        }
    }

    fn left(&self) -> Point {
        Point {
            x: self.0.x - 1,
            ..self.0
        }
    }
    fn right(&self) -> Point {
        Point {
            x: self.0.x + 1,
            ..self.0
        }
    }

    fn up(&self) -> Point {
        Point {
            y: self.0.y + 1,
            ..self.0
        }
    }
    fn down(&self) -> Point {
        // dbg!(self);
        Point {
            y: self.0.y - 1,
            ..self.0
        }
    }
}

fn parse_cubes(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|l| {
            let [x, y, z]: [isize; 3] = l
                .splitn(3, ',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Cube(Point { x, y, z })
        })
        .collect()
}

pub struct Day18;

impl Day for Day18 {
    fn part_1(&self, input: &str) -> String {
        let cubes = parse_cubes(input);
        let total = surface_area(&cubes);
        total.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let cubes = parse_cubes(input);
        let max_x = cubes.iter().map(|c| c.0.x).max().unwrap();
        let max_y = cubes.iter().map(|c| c.0.y).max().unwrap();
        let max_z = cubes.iter().map(|c| c.0.z).max().unwrap();

        let points: HashSet<Point> = cubes.iter().map(|c| c.0).collect();
        let mut total_droplet = 0;
        let mut possible: HashSet<Cube> = HashSet::new();
        for cube in cubes {
            if !points.contains(&cube.front()) {
                possible.insert(Cube(cube.front()));
                total_droplet += 1;
            }
            if !points.contains(&cube.back()) {
                possible.insert(Cube(cube.back()));
                total_droplet += 1;
            }
            if !points.contains(&cube.right()) {
                possible.insert(Cube(cube.right()));
                total_droplet += 1;
            }
            if !points.contains(&cube.left()) {
                possible.insert(Cube(cube.left()));
                total_droplet += 1;
            }
            if !points.contains(&cube.up()) {
                possible.insert(Cube(cube.up()));
                total_droplet += 1;
            }
            if !points.contains(&cube.down()) {
                possible.insert(Cube(cube.down()));
                total_droplet += 1;
            }
        }
        let outside = find_outside(&points, max_x, max_y, max_z);
        let mut inside: Vec<Cube> = possible
            .into_iter()
            .filter(|cube| !outside.contains(&cube.0) && !points.contains(&cube.0))
            .collect();
        inside.sort();
        dbg!(&inside);
        let total_inside = surface_area(&inside);
        let total = total_droplet - total_inside;
        total.to_string()
    }
}

fn find_outside(
    points: &HashSet<Point>,
    max_x: isize,
    max_y: isize,
    max_z: isize,
) -> HashSet<Point> {
    let mut visited = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_front(Point {
        x: -1,
        y: -1,
        z: -1,
    });
    while let Some(point @ Point { x, y, z }) = queue.pop_front() {
        if !visited.insert(point) {
            continue;
        }

        let mut p_x = point;
        p_x.x -= 1;
        if x >= -1 && !points.contains(&p_x) {
            queue.push_back(p_x);
        }
        p_x.x += 2;
        if x <= max_x + 1 && !points.contains(&p_x) {
            queue.push_back(p_x);
        }

        let mut p_y = point;
        p_y.y -= 1;
        if y >= -1 && !points.contains(&p_y) {
            queue.push_back(p_y);
        }
        p_y.y += 2;
        if y <= max_y + 1 && !points.contains(&p_y) {
            queue.push_back(p_y);
        }

        let mut p_z = point;
        p_z.z -= 1;
        if z >= -1 && !points.contains(&p_z) {
            queue.push_back(p_z);
        }
        p_z.z += 2;
        if z <= max_z + 1 && !points.contains(&p_z) {
            queue.push_back(p_z);
        }
    }
    visited
}

fn surface_area(cubes: &[Cube]) -> usize {
    let points: HashSet<Point> = cubes.iter().map(|c| c.0).collect();
    let mut total = 0;
    for cube in cubes {
        if !points.contains(&cube.front()) {
            total += 1;
        }
        if !points.contains(&cube.back()) {
            total += 1;
        }
        if !points.contains(&cube.right()) {
            total += 1;
        }
        if !points.contains(&cube.left()) {
            total += 1;
        }
        if !points.contains(&cube.up()) {
            total += 1;
        }
        if !points.contains(&cube.down()) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod day_18_tests {
    use super::*;

    #[test]
    fn get_simple_outside() {
        let lava: HashSet<Point> = vec![Point { x: 0, y: 0, z: 0 }].into_iter().collect();
        let outside = find_outside(&lava, 0, 0, 0);
        assert!(!outside.contains(&Point { x: 0, y: 0, z: 0 }));
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    assert!(outside.contains(&Point { x, y, z }));
                }
            }
        }
    }
}
