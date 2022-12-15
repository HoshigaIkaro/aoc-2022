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
    for new @ (new_left, new_right) in intervals.into_iter() {
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

pub struct Day15;

impl Day for Day15 {
    fn part_1(&self, input: &str) -> String {
        let sensors = parse_sensors(input);
        let intervals: Vec<(isize, isize)> =
            sensors.iter().filter_map(|s| s.h_interval(PART_ONE_ROW)).collect();
        merge_intervals(intervals)
            .into_iter()
            .map(|(left, right)| left.abs_diff(right + 1))
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let sensors = parse_sensors(input);

        let (x, y) = sensors
            .iter()
            .flat_map(|sensor| {
                let (x, y) = sensor.position;
                let start_x = (x - sensor.m_dist - 1).max(0);
                let end_x = x.min(MAX_DISTANCE);
                (start_x..=end_x).find_map(|n_x| {
                    let mut valid = None;
                    let delta = n_x - start_x;
                    let n_y = y + delta;
                    let point = (n_x, n_y);
                    if (0..=MAX_DISTANCE).contains(&n_y)
                        && valid_spot(&sensors, point)
                        && !valid_spot(&sensors, (n_x - 1, n_y))
                        && !valid_spot(&sensors, (n_x, n_y + 1))
                    {
                        valid = Some(point);
                    }
                    valid
                })
            })
            .next()
            .unwrap();
        let tuning = x as i64 * 4_000_000 + y as i64;
        tuning.to_string()
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
