use std::cmp::Ordering;

use super::Day;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Int(usize),
    List(Vec<Value>),
}

impl Value {
    fn new(input: &str) -> Self {
        if input.starts_with('[') {
            // list
            let mut values = Vec::new();
            let input = input.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            let mut buf = String::new();
            let mut level = 0;
            for c in input.chars() {
                match c {
                    '[' => {
                        level += 1;
                        buf.push('[');
                    }
                    ']' => {
                        level -= 1;
                        buf.push(']');
                    }
                    ',' => {
                        if level == 0 {
                            values.push(Value::new(&buf));
                            buf.clear();
                        } else {
                            buf.push(',');
                        }
                    }
                    c => buf.push(c),
                }
            }
            if !buf.is_empty() {
                values.push(Value::new(&buf));
            }
            Self::List(values)
        } else {
            // single
            Self::Int(input.parse().unwrap())
        }
    }

    fn new_list_single(int: usize) -> Self {
        Self::List(vec![Self::Int(int)])
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Int(left), Value::Int(right)) => left.cmp(right),
            (Value::Int(left), right) => Value::new_list_single(*left).cmp(right),
            (left, Value::Int(right)) => left.cmp(&Value::new_list_single(*right)),
            (Value::List(left), Value::List(right)) => {
                for i in 0..left.len().max(right.len()) {
                    match (left.get(i), right.get(i)) {
                        (Some(left), Some(right)) => match left.cmp(right) {
                            Ordering::Less => return Ordering::Less, // in order
                            Ordering::Equal => (),
                            Ordering::Greater => return Ordering::Greater, // not in order
                        },
                        (Some(_), None) => return Ordering::Greater, // right ran out first -> not in order
                        (None, Some(_)) => return Ordering::Less, // left ran out first -> in order
                        (None, None) => unreachable!(),
                    }
                }
                Ordering::Equal
            }
        }
    }
}
pub struct Day13;

impl Day for Day13 {
    fn part_1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .enumerate()
            .map(|(index, pair)| {
                
                let (left, right) = pair.split_once('\n').unwrap();
                let left = Value::new(left);
                let right = Value::new(right);
                if left.cmp(&right) == Ordering::Less {
                    index + 1
                } else {
                    0
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let one = Value::new("[[2]]");
        let two = Value::new("[[6]]");
        let mut values: Vec<Value> = input
            .lines()
            .filter_map(|s| if s.is_empty() {
                None
            } else {
                Some(Value::new(s))
            })
            .chain(vec![one.clone(), two.clone()])
            .collect();
        values.sort();
        values
            .into_iter()
            .enumerate()
            .filter(|(_, v)| v == &one || *v == two)
            .map(|(i, _)| i + 1)
            .product::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod day_13_tests {
    use super::*;

    #[test]
    fn new_int_value() {
        assert_eq!(Value::new("1"), Value::Int(1));
    }

    #[test]
    fn new_list_value() {
        assert_eq!(Value::new("[1]"), Value::List(vec![Value::Int(1)]));
    }

    #[test]
    fn new_list_multiple_values() {
        assert_eq!(
            Value::new("[1,2,3]"),
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );
    }

    #[test]
    fn test_simple_order() {
        // 1
        let left = Value::new("[1,1,3,1,1]");
        let right = Value::new("[1,1,5,1,1]");
        assert_eq!(left.cmp(&right), Ordering::Less);

        //2
        let left = Value::new("[[1],[2,3,4]]");
        let right = Value::new("[[1],4]");
        assert_eq!(left.cmp(&right), Ordering::Less);

        //3
        let left = Value::new("[9]");
        let right = Value::new("[[8,7,6]]");
        assert_eq!(left.cmp(&right), Ordering::Greater);

        // 4
        let left = Value::new("[[4,4],4,4]");
        let right = Value::new("[[4,4],4,4,4]");
        assert_eq!(left.cmp(&right), Ordering::Less);

        // 5
        let left = Value::new("[7,7,7,7]");
        let right = Value::new("[7,7,7]");
        assert_eq!(left.cmp(&right), Ordering::Greater);

        // 6
        let left = Value::new("[]");
        let right = Value::new("[3]");
        assert_eq!(left.cmp(&right), Ordering::Less);

        // 7
        let left = Value::new("[[[]]]");
        let right = Value::new("[[]]");
        assert_eq!(left.cmp(&right), Ordering::Greater);

        // 8
        let left = Value::new("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let right = Value::new("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }
}
