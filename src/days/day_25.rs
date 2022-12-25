use std::{iter::Sum, str::FromStr};

use super::Day;

#[derive(Debug)]
struct ParseSNAFUError;

#[derive(Debug, PartialEq, Eq)]
struct SNAFU(isize);

impl Sum for SNAFU {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.into_iter().map(|s| s.0).sum())
    }
}

impl FromStr for SNAFU {
    type Err = ParseSNAFUError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sum = 0;
        for (place, digit) in s.chars().rev().enumerate() {
            let value = match digit {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            };
            sum += value * 5_isize.pow(place as u32);
        }
        Ok(Self(sum))
    }
}

impl ToString for SNAFU {
    fn to_string(&self) -> String {
        let mut out = String::new();
        let mut num = self.0;
        let mut carry = 0;
        while num > 0 {
            let mut digit = num % 5 + carry;
            carry = 0;
            if digit > 2 {
                carry = 1;
                digit -= 5;
            }
            out += match digit {
                2 => "2",
                1 => "1",
                0 => "0",
                -1 => "-",
                -2 => "=",
                _ => panic!(),
            };

            num /= 5;
        }
        if carry == 1 {
            out += "1";
        }

        out.chars().rev().collect()
    }
}

pub struct Day25;

impl Day for Day25 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| SNAFU::from_str(line).unwrap())
            .sum::<SNAFU>()
            .to_string()
    }

    fn part_2(&self, _input: &str) -> String {
        "No Part 2.".to_string()
    }
}

#[cfg(test)]
mod day_25_tests {
    use super::*;

    #[test]
    fn from_basic() {
        assert_eq!(SNAFU::from_str("1").unwrap(), SNAFU(1));
        assert_eq!(SNAFU::from_str("1=11-2").unwrap(), SNAFU(2022));
        assert_eq!(SNAFU::from_str("1121-1110-1=0").unwrap(), SNAFU(314159265));
    }

    #[test]
    fn to_basic() {
        assert_eq!(SNAFU(1257).to_string(), "20012");
        assert_eq!(SNAFU(1747).to_string(), "1=-0-2");
    }
}
