use aoc::days::Day;
use criterion::{criterion_group, criterion_main, Criterion};
use paste::paste;

macro_rules! bench_day {
    ($day:literal) => {
        paste! {
            fn [<day_ $day>](c: &mut Criterion) {
                let input = load_input($day);
                let mut group = c.benchmark_group(concat!("day_", stringify!($day)));
                let day = get_day!($day);
                group.bench_with_input("part_1", &input, |b, input| {
                    b.iter(|| day.part_1(&input))
                });
                group.bench_with_input("part_2", &input, |b, input| {
                    b.iter(|| day.part_2(&input))
                });
                group.finish();
            }
        }
    };
}

macro_rules! get_day {
    ($day:literal) => {
        paste! {
            aoc::days::[<Day $day>]
        }
    };
}

fn load_input(day: u8) -> String {
    let path = format!("{}/inputs/day_{day}.txt", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path).unwrap()
}

fn day_16(c: &mut Criterion) {
    let input = load_input(16);
    let mut group = c.benchmark_group("day_16");
    let day = get_day!(16);
    group.bench_with_input("part_1", &input, |b, input| {
        b.iter(|| day.part_1(&input))
    });
    group.sample_size(10);
    group.bench_with_input("part_2", &input, |b, input| {
        b.iter(|| day.part_2(&input))
    });
    group.finish();
}

bench_day!(01);
bench_day!(02);
bench_day!(03);
bench_day!(04);
bench_day!(05);
bench_day!(06);
bench_day!(07);
bench_day!(08);
bench_day!(09);
bench_day!(10);
bench_day!(11);
bench_day!(12);
bench_day!(13);
bench_day!(14);
bench_day!(15);
// bench_day!(16);
bench_day!(17);
bench_day!(18);
bench_day!(19);
bench_day!(20);

criterion_group!(
    complete, day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10,
    day_11, day_12, day_13, day_14, day_15, day_16, day_17, day_18, day_19, day_20
);
criterion_main!(complete);
