use super::Day;

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
}

/// Returns true if no sensor can detect this point.
fn valid_spot(beacons: &[Sensor], point: Point) -> bool {
    beacons
        .iter()
        .all(|beacon| beacon.dist(point) > beacon.m_dist)
}

pub struct Day15;

impl Day for Day15 {
    fn part_1(&self, input: &str) -> String {
        let sensors = parse_sensors(input);
        let left_x = sensors
            .iter()
            .map(|b| b.position.0 - b.m_dist)
            .min()
            .unwrap();
        let right_x = sensors
            .iter()
            .map(|b| b.position.0 + b.m_dist)
            .max()
            .unwrap();

        let mut count = 0;
        for x in left_x..=right_x {
            let point = (x, 2_000_000);
            for sensor in &sensors {
                // beacon cannot be placed here
                if sensor.dist(point) <= sensor.m_dist && sensor.closest != point {
                    count += 1;
                    break;
                }
            }
        }
        count.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let sensors = parse_sensors(input);

        let mut valid = (0, 0);
        for one in &sensors {
            let (x, y) = one.position;
            let mut d_y = 0;
            for n_x in x - one.m_dist - 1..x.min(4_000_000) {
                if n_x < 0 {
                    d_y += 1;
                    continue;
                }

                let point = (n_x, y + d_y);
                if point.1 <= 4_000_000 && valid_spot(&sensors, point) {
                    valid = point;
                    break;
                }

                let point = (n_x, y - d_y);
                if point.1 >= 0 && valid_spot(&sensors, point) {
                    valid = point;
                    break;
                }

                d_y += 1;
            }
        }
        let (x, y) = valid;
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
}
