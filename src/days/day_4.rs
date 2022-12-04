use super::Day;

pub struct Day4;

impl Day for Day4 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                line.split(',')
                    .flat_map(|range| range.split('-'))
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|arr| {
                (arr[0] >= arr[2] && arr[1] <= arr[3]) || (arr[2] >= arr[0] && arr[3] <= arr[1])
            })
            .count()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                line.split(',')
                    .flat_map(|range| range.split('-'))
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|arr| arr[0] <= arr[3] && arr[2] <= arr[1])
            .count()
            .to_string()
    }
}
