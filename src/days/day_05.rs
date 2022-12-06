use super::Day;

pub struct Day05;

impl Day for Day05 {
    fn part_1(&self, input: &str) -> String {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        let mut cmap = get_crates(crates);
        for line in instructions.lines() {
            let (num, origin, target) = parse_instruction(line);
            for _ in 0..num {
                let v = cmap.get_mut(origin - 1).unwrap().pop().unwrap();
                cmap.get_mut(target - 1).unwrap().push(v);
            }
        }
        cmap.iter().map(|v| v.last().unwrap()).collect()
    }

    fn part_2(&self, input: &str) -> String {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        let mut cmap = get_crates(crates);
        for line in instructions.lines() {
            let (num, origin, target) = parse_instruction(line);
            let v = cmap.get_mut(origin - 1).unwrap();
            let b = v.split_off(v.len() - num);
            cmap.get_mut(target - 1).unwrap().extend(b.into_iter());
        }
        cmap.iter().map(|v| v.last().unwrap()).collect()
    }
}

fn get_crates(input: &str) -> Vec<Vec<char>> {
    let (left, right) = input.rsplit_once('\n').unwrap();
    let num_crates = right.split_whitespace().last().unwrap().parse().unwrap();
    let mut cmap: Vec<Vec<char>> = vec![Vec::new(); num_crates];
    for line in left.lines() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                cmap[i].insert(0, c);
            }
        }
    }
    cmap
}

fn parse_instruction(line: &str) -> (usize, usize, usize) {
    let mut s = line.split_whitespace();
    s.next();
    let num = s.next().unwrap().parse().unwrap();
    s.next();
    let origin = s.next().unwrap().parse().unwrap();
    s.next();
    let target = s.next().unwrap().parse().unwrap();
    (num, origin, target)
}
