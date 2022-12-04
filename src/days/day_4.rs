use super::Day;

pub struct Day4;

impl Day for Day4 {
    fn part_1(&self, input: &str) -> String {
        let mut total = 0;
        for line in input.lines() {
            let (one, two) = line.split_once(',').unwrap();
            let (l_1, h_1) = get_range(one);
            let (l_2, h_2) = get_range(two);
            if (l_1 >= l_2 && h_1 <= h_2) || (l_2 >= l_1 && h_2 <= h_1) { 
                total += 1;
            }
        }
        total.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut total = 0;
        for line in input.lines() {
            let (one, two) = line.split_once(',').unwrap();
            let (l_1, h_1) = get_range(one);
            let (l_2, h_2) = get_range(two);
            if (l_1 >= l_2 && l_1 <= h_2)
            || (l_2 >= l_1 && l_2 <= h_1)
             { 
                total += 1;
            }
        }
        total.to_string()
    }
}

fn get_range(s: &str) -> (u8, u8) {
    let (a, b) = s.split_once('-').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}