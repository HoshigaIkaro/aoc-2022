use std::fmt::Display;

use super::Day;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Nothing,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Open => '.',
            Tile::Wall => '#',
            Tile::Nothing => ' ',
        };
        write!(f, "{c}")
    }
}

type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Literal(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Debug)]
struct Grove {
    board: Vec<Vec<Tile>>,
    x: usize,
    y: usize,
    direction: Direction,
}

impl Grove {
    fn new(map_input: &str) -> Self {
        let mut board = Vec::new();
        let mut max = 0;
        for line in map_input.lines() {
            let mut temp = Vec::new();
            temp.push(Tile::Nothing);
            max = max.max(line.len());
            for tile in line.chars() {
                let tile = match tile {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    ' ' => Tile::Nothing,
                    _ => unreachable!(),
                };
                temp.push(tile);
            }
            temp.extend(vec![Tile::Nothing; max - line.len()]);
            temp.push(Tile::Nothing);
            board.push(temp);
        }
        board.push(vec![Tile::Nothing; board[0].len()]);
        board.insert(0, vec![Tile::Nothing; board[0].len()]);
        let start = board[1].iter().position(|i| *i == Tile::Open).unwrap();
        Self {
            board,
            x: start,
            y: 1,
            direction: Direction::Right,
        }
    }

    fn get(&self, (x, y): Point) -> Tile {
        self.board[y][x]
    }

    fn open_point(&self, point: Point) -> bool {
        self.get(point) == Tile::Open
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn advance(&mut self, step: Step, is_cube: bool) {
        match step {
            Step::Literal(steps) => {
                if is_cube {
                    for _ in 0..steps {
                        self.move_forward_cube();
                    }
                } else {
                    for _ in 0..steps {
                        self.move_forward();
                    }
                }
            }
            Step::TurnLeft => self.turn_left(),
            Step::TurnRight => self.turn_right(),
        }
    }

    fn move_forward(&mut self) {
        let mut new_point = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };

        if self.get(new_point) == Tile::Nothing {
            match self.direction {
                Direction::Up => {
                    new_point.1 = self.board.len()
                        - 1
                        - self
                            .board
                            .iter()
                            .rev()
                            .position(|row| row[self.x] != Tile::Nothing)
                            .unwrap();
                }
                Direction::Down => {
                    new_point.1 = self
                        .board
                        .iter()
                        .position(|row| row[self.x] != Tile::Nothing)
                        .unwrap();
                }
                Direction::Left => {
                    new_point.0 = self.board[self.y].len()
                        - 1
                        - self.board[self.y]
                            .iter()
                            .rev()
                            .position(|tile| *tile != Tile::Nothing)
                            .unwrap();
                }
                Direction::Right => {
                    new_point.0 = self.board[self.y]
                        .iter()
                        .position(|tile| *tile != Tile::Nothing)
                        .unwrap();
                }
            }
        }
        if self.open_point(new_point) {
            (self.x, self.y) = new_point;
        }
    }

    fn move_forward_cube(&mut self) {
        let mut new_point = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        let mut new_direction = self.direction;
        if self.get(new_point) == Tile::Nothing {
            match self.direction {
                Direction::Up => match self.x {
                    1..=50 => {
                        new_point.0 = 51;
                        new_point.1 = 50 + self.x;
                        new_direction = Direction::Right;
                    }
                    51..=100 => {
                        new_point.0 = 1;
                        new_point.1 = 100 + self.x;
                        new_direction = Direction::Right;
                    }
                    101..=150 => {
                        new_point.0 = self.x - 100;
                        new_point.1 = 200;
                    }
                    _ => unreachable!(),
                },
                Direction::Down => match self.x {
                    1..=50 => {
                        new_point.0 += 100;
                        new_point.1 = 1;
                    }
                    51..=100 => {
                        new_point.0 = 50;
                        new_point.1 = self.x + 100;
                        new_direction = Direction::Left;
                    }
                    101..=150 => {
                        new_point.0 = 100;
                        new_point.1 = self.x - 50;
                        new_direction = Direction::Left;
                    }
                    _ => unreachable!(),
                },
                Direction::Left => match self.y {
                    1..=50 => {
                        new_point.0 = 1;
                        new_point.1 = 151 - self.y;
                        new_direction = Direction::Right;
                    }
                    51..=100 => {
                        new_point.0 = self.y - 50;
                        new_point.1 = 101;
                        new_direction = Direction::Down;
                    }
                    101..=150 => {
                        new_point.0 = 51;
                        new_point.1 = 151 - self.y;
                        new_direction = Direction::Right;
                    }
                    151..=200 => {
                        new_point.0 = self.y - 100;
                        new_point.1 = 1;
                        new_direction = Direction::Down;
                    }
                    _ => unreachable!(),
                },
                Direction::Right => match self.y {
                    1..=50 => {
                        new_point.0 = 100;
                        new_point.1 = 151 - self.y;
                        new_direction = Direction::Left;
                    }
                    51..=100 => {
                        new_point.0 = self.y + 50;
                        new_point.1 = 50;
                        new_direction = Direction::Up;
                    }
                    101..=150 => {
                        new_point.0 = 150;
                        new_point.1 = 151 - self.y;
                        new_direction = Direction::Left;
                    }
                    151..=200 => {
                        new_point.0 = self.y - 100;
                        new_point.1 = 150;
                        new_direction = Direction::Up;
                    }
                    _ => unreachable!(),
                },
            }
        }
        if self.open_point(new_point) {
            (self.x, self.y) = new_point;
            self.direction = new_direction;
        }
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.board.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.y == y && self.x == x {
                    let c = match self.direction {
                        Direction::Up => "^",
                        Direction::Down => "v",
                        Direction::Left => "<",
                        Direction::Right => ">",
                    };
                    write!(f, "{c}")?;
                } else {
                    write!(f, "{tile}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Day22;

impl Day for Day22 {
    fn part_1(&self, input: &str) -> String {
        let (map_input, steps_input) = input.split_once("\n\n").unwrap();
        let mut grove = Grove::new(map_input);
        for step in parse_steps(steps_input.trim()) {
            grove.advance(step, false);
        }
        let row = grove.y * 1000;
        let col = grove.x * 4;
        let facing = grove.direction as usize;

        let password = row + col + facing;
        password.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (map_input, steps_input) = input.split_once("\n\n").unwrap();
        let mut grove = Grove::new(map_input);
        for step in parse_steps(steps_input.trim()) {
            grove.advance(step, true);
        }
        let row = grove.y * 1000;
        let col = grove.x * 4;
        let facing = grove.direction as usize;

        let password = row + col + facing;
        password.to_string()
    }
}

fn parse_steps(input: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut buf = String::new();
    for c in input.chars() {
        match c {
            'R' => {
                steps.push(Step::Literal(buf.parse().unwrap()));
                buf.clear();
                steps.push(Step::TurnRight);
            }
            'L' => {
                steps.push(Step::Literal(buf.parse().unwrap()));
                buf.clear();
                steps.push(Step::TurnLeft);
            }
            c => buf.push(c),
        }
    }
    // last step
    steps.push(Step::Literal(buf.parse().unwrap()));
    steps
}
