use super::Day;

#[derive(Debug)]
enum Op {
    Noop,
    Addx(isize),
}

enum State {
    Idle,
    Adding(isize),
}

pub struct Day10;

impl Day for Day10 {
    fn part_1(&self, input: &str) -> String {
        let mut ops = parse_to_iter(input);

        let mut x = 1;
        let mut state = State::Idle;

        let mut sum = 0;
        let mut check = |cycle: isize, x: isize| {
            if (cycle - 20) % 40 == 0 {
                sum += cycle * x;
            }
        };

        for cycle in 1..=220 {
            match state {
                State::Idle => {
                    check(cycle, x);
                    match ops.next().unwrap() {
                        Op::Noop => (),
                        Op::Addx(v) => state = State::Adding(v),
                    }
                }
                State::Adding(v) => {
                    check(cycle, x);
                    state = State::Idle;
                    x += v;
                }
            }
        }
        sum.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut ops = parse_to_iter(input);

        let mut x: isize = 1;
        let mut state = State::Idle;

        let mut out = String::new();
        for _ in 0..6 {
            for col in 0..40 {
                if x.abs_diff(col) <= 1 {
                    out.push('#');
                } else {
                    out.push(' ');
                }
                match state {
                    State::Idle => match ops.next().unwrap() {
                        Op::Noop => (),
                        Op::Addx(v) => state = State::Adding(v),
                    },
                    State::Adding(v) => {
                        state = State::Idle;
                        x += v;
                    }
                }
            }
            out.push('\n');
        }
        out
    }
}

fn parse_to_iter(input: &str) -> impl Iterator<Item = Op> + '_ {
    input.lines().map(|line| match line {
        "noop" => Op::Noop,
        _ => Op::Addx(line.split_at(5).1.parse().unwrap()),
    })
}
