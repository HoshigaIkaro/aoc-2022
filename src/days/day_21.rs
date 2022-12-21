use std::collections::HashMap;

use super::Day;

type Monkeys<'a> = HashMap<&'a str, Value<'a>>;

enum Value<'a> {
    Unary(usize),
    Binary(Op, &'a str, &'a str),
}

impl<'a> Value<'a> {
    fn get(&self, monkeys: &Monkeys) -> usize {
        match self {
            Value::Unary(value) => *value,
            Value::Binary(op, left, right) => {
                let left = monkeys[left].get(monkeys);
                let right = monkeys[right].get(monkeys);
                match op {
                    Op::Add => left + right,
                    Op::Minus => left - right,
                    Op::Multiply => left * right,
                    Op::Divide => left / right,
                }
            }
        }
    }

    fn get_unless_human(&self, monkeys: &Monkeys) -> Option<usize> {
        match self {
            Value::Unary(value) => Some(*value),
            Value::Binary(op, left, right) => {
                if *left == "humn" || *right == "humn" {
                    None
                } else {
                    let left = monkeys[left].get_unless_human(monkeys);
                    let right = monkeys[right].get_unless_human(monkeys);
                    if left.is_none() || right.is_none() {
                        None
                    } else {
                        let left = left.unwrap();
                        let right = right.unwrap();
                        Some(match op {
                            Op::Add => left + right,
                            Op::Minus => left - right,
                            Op::Multiply => left * right,
                            Op::Divide => left / right,
                        })
                    }
                }
            }
        }
    }

    fn contains(&self, target: &'a str) -> bool {
        match self {
            Value::Unary(_) => false,
            Value::Binary(_, left, right) => *left == target || *right == target,
        }
    }
}

enum Op {
    Add,
    Minus,
    Multiply,
    Divide,
}

impl Op {
    fn reverse(&self) -> Self {
        match self {
            Op::Add => Op::Minus,
            Op::Minus => Op::Add,
            Op::Multiply => Op::Divide,
            Op::Divide => Op::Multiply,
        }
    }
}

pub struct Day21;

impl Day for Day21 {
    fn part_1(&self, input: &str) -> String {
        let monkeys = parse_input(input);
        monkeys["root"].get(&monkeys).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let monkeys = parse_input(input);
        if let Value::Binary(_, left, right) = monkeys["root"] {
            let (mut target_value, container) =
                if let Some(value) = monkeys[left].get_unless_human(&monkeys) {
                    (value, right)
                } else {
                    (monkeys[right].get(&monkeys), left)
                };
            let mut container = container;
            loop {
                // dbg!(container);
                let value = &monkeys[container];
                if value.contains("humn") {
                    if let Value::Binary(op, left, right) = value {
                        if *left != "humn" {
                            let value = monkeys[left].get(&monkeys);
                            target_value = match op {
                                Op::Add => target_value - value,
                                Op::Minus => value - target_value,
                                Op::Multiply => target_value / value,
                                Op::Divide => value / target_value,
                            };
                        } else {
                            let value = monkeys[right].get(&monkeys);
                            target_value = match op {
                                Op::Add => target_value - value,
                                Op::Minus => target_value + value,
                                Op::Multiply => target_value / value,
                                Op::Divide => value * target_value,
                            };
                        }
                        break;
                    }
                }
                match value {
                    Value::Unary(_) => todo!(),
                    Value::Binary(op, left, right) => {
                        if let Some(value) = monkeys[left].get_unless_human(&monkeys) {
                            // dbg!("left does not have human");
                            target_value = match op {
                                Op::Add => target_value - value,
                                Op::Minus => value - target_value,
                                Op::Multiply => target_value / value,
                                Op::Divide => value / target_value,
                            };
                            container = *right;
                        } else {
                            let value = monkeys[right].get(&monkeys);
                            target_value = match op {
                                Op::Add => target_value - value,
                                Op::Minus => target_value + value,
                                Op::Multiply => target_value / value,
                                Op::Divide => value * target_value,
                            };
                            container = *left;
                        }
                    }
                }
            }
            return target_value.to_string();
        }
        unreachable!()
    }
}

fn parse_input(input: &str) -> Monkeys {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (monkey, value) = line.split_once(": ").unwrap();
        let value = value.split_whitespace().collect::<Vec<_>>();
        let value = if value.len() == 1 {
            Value::Unary(value[0].parse().unwrap())
        } else {
            let op = match value[1] {
                "+" => Op::Add,
                "-" => Op::Minus,
                "*" => Op::Multiply,
                "/" => Op::Divide,
                _ => unreachable!(),
            };
            let left = value[0];
            let right = value[2];
            Value::Binary(op, left, right)
        };
        monkeys.insert(monkey, value);
    }

    monkeys
}
