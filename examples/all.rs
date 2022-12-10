use std::time::Instant;

use aoc::run_day;

fn main() {
    let start = Instant::now();
    for day in 1..=9 {
        run_day(day);
    }
    let time = start.elapsed();
    println!("{} µs or {} ms", time.as_micros(), time.as_millis());
}
