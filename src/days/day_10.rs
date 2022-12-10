use super::Day;

#[derive(Debug)]
enum Op {
    Noop,
    Add(isize),
}

pub struct Day10;

impl Day for Day10 {
    fn part_1(&self, input: &str) -> String {
        let mut cycle = 0;
        let mut x = 1;
        let mut ops = parse_to_iter(input);
        let mut out: isize = 0;
        let mut counter = 0;
        let mut buf = None;
        loop {
            if counter > 0 {
                counter -= 1;
            } else if let Some(v) = buf {
                x += v;
                buf = None;
                counter = 0;
                match ops.next() {
                    Some(op) => match op {
                        Op::Noop => (),
                        Op::Add(v) => {
                            buf = Some(v);
                            counter = 1;
                        }
                    },
                    None => break,
                }
            } else {
                match ops.next() {
                    Some(op) => match op {
                        Op::Noop => (),
                        Op::Add(v) => {
                            buf = Some(v);
                            counter = 1;
                        }
                    },
                    None => break,
                }
            }
            cycle += 1;

            if (cycle - 20) % 40 == 0 {
                out += cycle as isize * x;
            }
        }
        out.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut x = 1;
        let mut ops = parse_to_iter(input);
        let mut counter = 0;
        let mut buf = None;
        let mut out = String::new();
        for _ in 0..6 {
            for cycle in 0..40 {
                if counter > 0 {
                    counter -= 1;
                } else if let Some(v) = buf {
                    x += v;
                    buf = None;
                    counter = 0;
                    match ops.next() {
                        Some(op) => match op {
                            Op::Noop => (),
                            Op::Add(v) => {
                                buf = Some(v);
                                counter = 1;
                            }
                        },
                        None => break,
                    }
                } else {
                    match ops.next() {
                        Some(op) => match op {
                            Op::Noop => (),
                            Op::Add(v) => {
                                buf = Some(v);
                                counter = 1;
                            }
                        },
                        None => break,
                    }
                }

                if x.abs_diff(cycle) <= 1 {
                    out += "#";
                } else {
                    out += ".";
                }
            }
            out += "\n";
        }
        out
    }
}

fn parse_to_iter(input: &str) -> impl Iterator<Item = Op> + '_ {
    input.lines().map(|line| match line {
        "noop" => Op::Noop,
        _ => Op::Add(line.split_at(5).1.parse().unwrap()),
    })
}
