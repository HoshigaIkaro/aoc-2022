use super::Day;

type Point = (usize, usize);

//- Stores the point at the lower left corner
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Rock {
    rock_type: RockType,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum RockType {
    Horizontal = 0,
    Cross = 1,
    Angle = 2,
    Vertical = 3,
    Square = 4,
}

impl RockType {
    fn set_next_type(&mut self) {
        *self = match self {
            RockType::Horizontal => Self::Cross,
            RockType::Cross => Self::Angle,
            RockType::Angle => Self::Vertical,
            RockType::Vertical => Self::Square,
            RockType::Square => Self::Horizontal,
        }
    }
}

impl Rock {
    fn new(x: usize, y: usize) -> Self {
        Self {
            rock_type: RockType::Horizontal,
            x,
            y,
        }
    }

    /// Finds the points of the rock based on the lower left point of the bounding box
    fn get_points(&self) -> Vec<Point> {
        let (x, y) = (self.x, self.y);
        match self.rock_type {
            RockType::Horizontal => (0..4).map(|delta| (x + delta, y)).collect(),
            RockType::Cross => vec![
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y),
                (x + 1, y + 2),
            ],
            RockType::Angle => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            RockType::Vertical => (0..4).map(|delta| (x, y + delta)).collect(),
            RockType::Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }

    fn set_next_type(&mut self) {
        self.rock_type.set_next_type();
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Air,
    Rock,
}

#[derive(Debug)]
struct Chamber {
    grid: Vec<Tile>,
    height: usize,
    rock: Rock,
    height_before_floor: usize,
}

fn to_index((x, y): Point) -> usize {
    y * 7 + x
}

#[allow(dead_code)]
impl Chamber {
    fn new() -> Self {
        Self {
            grid: vec![Tile::Air; 7],
            height: 1,
            rock: Rock::new(2, 3),
            height_before_floor: 0,
        }
    }

    fn get(&self, point: Point) -> Tile {
        *self.grid.get(to_index(point)).unwrap_or(&Tile::Air)
    }

    fn is_occupied(&self, point: Point) -> bool {
        self.get(point) == Tile::Rock
    }

    fn set(&mut self, (x, y): Point) {
        if y > self.height - 1 {
            // need to add row
            self.height = y + 1;
            self.grid
                .extend(vec![Tile::Air; (self.height + 1 - self.grid.len() / 7) * 7]);
        }
        // below height of container
        self.grid[to_index((x, y))] = Tile::Rock;
    }

    fn move_down(&mut self) -> bool {
        let landed = self
            .rock
            .get_points()
            .into_iter()
            .any(|(x, y)| y == 0 || self.is_occupied((x, y - 1)));
        if landed {
            // set the rock in stone
            for point in self.rock.get_points() {
                self.set(point);
            }
            // if self.rock.y > 10 {
            //     if let Some(new_floor) = self.new_floor_level(self.rock.y) {
            //         self.remove_until_new_floor(new_floor);
            //     }
            // }
            // change to next type of rock
            self.rock.set_next_type();
            // reset position;
            self.rock.y = self.height + 3; // offset one in the example
            self.rock.x = 2; // rule from the example
        } else {
            self.rock.y -= 1;
        }
        landed
    }

    fn move_right(&mut self) {
        let bordering_right = self
            .rock
            .get_points()
            .into_iter()
            .any(|(x, y)| x == 6 || self.is_occupied((x + 1, y)));
        if !bordering_right {
            self.rock.x += 1;
        }
    }

    fn move_left(&mut self) {
        let bordering_left = self
            .rock
            .get_points()
            .into_iter()
            .any(|(x, y)| x == 0 || self.is_occupied((x - 1, y)));
        if !bordering_left {
            self.rock.x -= 1;
        }
    }

    /// Checks three rows, starting from at most one underneath the provided level.
    fn new_floor_level(&self, y: usize) -> Option<usize> {
        (y.saturating_sub(1)..=y + 1).find(|&y| (0..7).all(|x| self.is_occupied((x, y))))
    }

    fn remove_until_new_floor(&mut self, new_floor: usize) {
        self.height_before_floor += new_floor;
        self.grid = self.grid.split_off(new_floor * 7);
        // change view
        self.height -= new_floor;
        // self.rock.y -= new_floor;
    }

    fn total_height(&self) -> usize {
        self.height_before_floor + self.height
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rock_points = self.rock.get_points();
        for y in (0..self.height + 3).rev() {
            for x in 0..7 {
                if rock_points.contains(&(x, y)) {
                    write!(f, "@")?;
                } else {
                    let tile = self.get((x, y));
                    let out = match tile {
                        Tile::Air => '.',
                        Tile::Rock => '#',
                    };
                    write!(f, "{}", out)?;
                }
            }
            write!(f, " {y}")?;
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Day17;

impl Day for Day17 {
    fn part_1(&self, input: &str) -> String {
        let mut ops = input.chars().cycle();
        let mut chamber = Chamber::new();
        let mut count: usize = 0;
        while count < 2022 {
            match ops.next().unwrap() {
                '>' => chamber.move_right(),
                '<' => chamber.move_left(),
                _ => unreachable!(),
            }
            if chamber.move_down() {
                count += 1;
            }
        }

        chamber.height.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut ops = input.chars().cycle();
        let mut op_index = 0;
        let mut chamber = Chamber::new();
        let mut states = Vec::new();
        let mut total_heights = Vec::new();
        let (offset_index, cycle_size) = loop {
            match ops.next().unwrap() {
                '>' => chamber.move_right(),
                '<' => chamber.move_left(),
                _ => unreachable!(),
            }

            if chamber.move_down() {
                // rock has landed
                total_heights.push(chamber.total_height());
                let state = (
                    chamber
                        .grid
                        .iter()
                        .copied()
                        .rev()
                        .take(20)
                        .collect::<Vec<_>>(),
                    chamber.rock.rock_type,
                    op_index,
                );
                if states.contains(&state) {
                    let offset_index = states.iter().position(|s| *s == state).unwrap();
                    break (offset_index, states.len() - offset_index);
                }
                states.push(state);
            }
            op_index = (op_index + 1) % input.len();
        };
        let offset = offset_index + 1;
        let offset_height = total_heights[offset_index];
        let single_cycle_height = total_heights.last().unwrap() - offset_height;
        let cycles = (1_000_000_000_000 - offset) / cycle_size;
        let cycle_height = single_cycle_height * cycles;
        let remaining = 1_000_000_000_000 - (cycles * cycle_size) - offset;
        let remaining_height =
            total_heights[offset_index + remaining] - total_heights[offset_index];
        let total_height = offset_height + cycle_height + remaining_height;
        total_height.to_string()
    }
}

#[cfg(test)]
mod day_17_tests {
    use super::*;

    #[test]
    fn test_move_down() {
        let mut chamber = Chamber::new();
        chamber.move_down();
        chamber.move_down();
        chamber.move_down();
        chamber.move_down(); // lands here
        for x in 2..6 {
            assert!(chamber.is_occupied((x, 0)))
        }
        assert_eq!(chamber.rock.x, 2);
        assert_eq!(chamber.rock.y, 4);
    }

    #[test]
    fn move_two_down() {
        let mut chamber = Chamber::new();
        for _ in 0..8 {
            chamber.move_down();
        }
        assert_eq!(chamber.height, 4)
    }

    #[test]
    fn chamber_new_floor() {
        let mut chamber = Chamber::new();
        chamber.grid = vec![vec![Tile::Air; 7], vec![Tile::Rock; 7]]
            .into_iter()
            .flatten()
            .collect();
        chamber.rock.y = 4;
        chamber.height = 3;
        let new_floor = chamber.new_floor_level(1).unwrap();
        println!("{chamber}");
        chamber.remove_until_new_floor(new_floor);
        println!("{chamber}");
        // height of 1 below the floor
        assert_eq!(chamber.height_before_floor, 1);
        assert_eq!(chamber.height, 2);
        assert_eq!(chamber.grid, vec![Tile::Rock; 7]);
        assert_eq!(chamber.total_height(), 3);
        assert_eq!(chamber.rock.y, 3);
    }
}
