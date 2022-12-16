use super::Day;

const PART_ONE_ROW: isize = 2_000_000;
const MAX_DISTANCE: isize = 4_000_000;

type Point = (isize, isize);

struct Sensor {
    /// Sensor
    position: Point,
    /// Beacon
    closest: Point,
    /// Manhattan distance
    m_dist: isize,
}

#[allow(clippy::cast_sign_loss)]
impl Sensor {
    fn new(position @ (p_x, p_y): Point, closest @ (c_x, c_y): Point) -> Self {
        let m_dist = p_x.abs_diff(c_x) + p_y.abs_diff(c_y);
        let m_dist = m_dist as isize;
        Self {
            position,
            closest,
            m_dist,
        }
    }

    /// Gets the manhattan distance from the sensor to the point.
    fn dist(&self, (o_x, o_y): Point) -> isize {
        let m_dist = self.position.0.abs_diff(o_x) + self.position.1.abs_diff(o_y);
        m_dist as isize
    }

    /// Gets the possible horizontal interval after moving `y` units up or down.
    ///
    /// None is returned if the distance to the row is too far.
    /// The left and right bounds are inclusive.
    fn h_interval(&self, y: isize) -> Option<(isize, isize)> {
        let sensor_y = self.position.1;
        let delta_y = sensor_y.abs_diff(y) as isize;
        if self.m_dist <= delta_y {
            return None;
        }
        let width = self.m_dist - delta_y;
        let x = self.position.0;
        let (left, right) = (x - width, x + width);

        let (beacon_x, beacon_y) = self.closest;
        let interval = if beacon_y == y && left <= beacon_x && beacon_x <= right {
            let beacon_left = beacon_x;

            // beacon must be either left or right end
            if beacon_left == left {
                (left + 1, right)
            } else {
                (left, right - 1)
            }
        } else {
            (left, right)
        };
        Some(interval)
    }
}

/// Returns true if no sensor can detect this point.
fn valid_spot(sensors: &[Sensor], point: Point) -> bool {
    sensors
        .iter()
        .all(|sensor| sensor.dist(point) > sensor.m_dist)
}

fn merge_intervals(intervals: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut intervals = intervals;
    intervals.sort_unstable();

    let mut merged = vec![intervals.remove(0)];
    for new @ (new_left, new_right) in intervals {
        let last @ (last_left, last_right) = merged.pop().unwrap();
        // overlapping section
        if last_left <= new_right && last_right >= new_left {
            merged.push((last_left.min(new_left), last_right.max(new_right)));
        } else {
            merged.push(last);
            merged.push(new);
        }
    }

    merged
}

#[derive(Debug, PartialEq)]
struct Line {
    slope: isize,
    y_intercept: isize,
}

pub struct Day15;

impl Day for Day15 {
    fn part_1(&self, input: &str) -> String {
        let sensors = parse_sensors(input);
        let intervals: Vec<(isize, isize)> = sensors
            .iter()
            .filter_map(|s| s.h_interval(PART_ONE_ROW))
            .collect();
        merge_intervals(intervals)
            .into_iter()
            .map(|(left, right)| left.abs_diff(right + 1))
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let sensors = parse_sensors(input);

        let mut positives: Vec<Line> = Vec::new();
        let mut negatives: Vec<Line> = Vec::new();
        for one in &sensors {
            for two in &sensors {
                if one.dist(two.position) == one.m_dist + two.m_dist + 2 {
                    // println!("{:?} {:?}", one.position, two.position);
                    let (x_1, y_1) = one.position;
                    let (x_2, y_2) = two.position;

                    let delta_y = y_2 - y_1;
                    let delta_x = x_2 - x_1;
                    let slope = -delta_y.signum() * delta_x.signum();
                    let center_y_intercept = y_1 - slope * x_1;
                    let delta_y_intercept = slope.signum() * (one.m_dist + 1);
                    let y_intercept = center_y_intercept + delta_y_intercept;

                    let line = Line { slope, y_intercept };
                    match slope.signum() {
                        1 => positives.push(line),
                        -1 => negatives.push(line),
                        _ => unreachable!(),
                    }
                }
            }
        }

        for one in positives {
            let b_p = one.y_intercept;
            for Line {
                slope,
                y_intercept: b_n,
            } in &negatives
            {
                // let b be the y-intercept
                // y_p = x + b_p
                // y_n = -x + b_n
                // y_p = y_n -> x + b_p = -x + b_n
                // 2x = b_n - b_p -> x = (b_n - b_p) / 2
                let x = (b_n - b_p) / 2;

                if !(0..=MAX_DISTANCE).contains(&x) {
                    continue;
                }
                let y = slope * x + b_n;
                if !(0..=MAX_DISTANCE).contains(&y) {
                    continue;
                }
                
                let point = (x, y);
                if valid_spot(&sensors, point) {
                    let tuning = x as i64 * 4_000_000 + y as i64;
                    return tuning.to_string();
                }
            }
        }
        todo!()
    }
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Sensor at ").unwrap();
            let (sensor, closest) = line.split_once(": closest beacon is at ").unwrap();

            let sensor = sensor.split_once(", ").unwrap();
            let sensor: (isize, isize) = (
                sensor.0.strip_prefix("x=").unwrap().parse().unwrap(),
                sensor.1.strip_prefix("y=").unwrap().parse().unwrap(),
            );

            let closest = closest.split_once(", ").unwrap();
            let closest: (isize, isize) = (
                closest.0.strip_prefix("x=").unwrap().parse().unwrap(),
                closest.1.strip_prefix("y=").unwrap().parse().unwrap(),
            );

            Sensor::new(sensor, closest)
        })
        .collect()
}

#[cfg(test)]
mod day_15_tests {
    use super::*;

    #[test]
    fn can_create_sensor() {
        let point = (8, 7);
        let closest = (2, 10);
        let sensor = Sensor::new(point, closest);
        assert_eq!(sensor.m_dist, 9);
    }

    #[test]
    fn can_get_horizontal_interval() {
        let point = (8, 7);
        let closest = (2, 10);
        let sensor = Sensor::new(point, closest);
        assert_eq!(sensor.h_interval(7), Some((-1, 17)));
    }

    #[test]
    fn correct_interval_with_edge_beacon() {
        let point = (0, 0);
        let closest = (1, 4);
        let sensor = Sensor::new(point, closest);
        assert_eq!(sensor.h_interval(4), Some((-1, 0)));

        let closest = (-1, 4);
        let sensor = Sensor::new(point, closest);
        assert_eq!(sensor.h_interval(4), Some((0, 1)));
    }

    #[test]
    fn correct_interval_merging() {
        let intervals = vec![(0, 10), (5, 12), (5, 20)];
        let merged = merge_intervals(intervals);
        assert_eq!(merged, vec![(0, 20)]);
    }
}
