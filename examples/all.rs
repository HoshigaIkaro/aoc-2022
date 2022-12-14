use owo_colors::OwoColorize;
use std::time::{Duration, Instant};

use aoc::run_day;

fn main() {
    let mut timings = Vec::new();
    for day in 1..=14 {
        let start = Instant::now();
        run_day(day);
        timings.push(start.elapsed());
    }
    let mut total = Duration::default();
    for (day, time) in timings.into_iter().enumerate() {
        total += time;
        println!(
            "{}",
            format!("Day {: >2} : {: >5} µs", day + 1, time.as_micros()).fg_rgb::<186, 187, 241>()
        );
    }
    println!();
    println!(
        "{}",
        format!(
            "Total time (including input and output):\n {} µs or {} ms",
            total.as_micros(),
            total.as_millis()
        )
        .fg_rgb::<140, 170, 238>()
    );
}
