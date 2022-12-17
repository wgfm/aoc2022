use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Range, RangeInclusive},
    str::FromStr,
};

use crate::util::{BoxError, Result};

//#[test]
fn test_one() {
    eprintln!("{}", one());
    assert!(false);
}

#[test]
fn test_two() {
    eprintln!("{}", two());
    assert!(false);
}

fn one() -> usize {
    let sensors = read_input().unwrap();

    let mut xes = HashSet::new();
    for sensor in sensors {
        for x in sensor.range_at_y(2_000_000) {
            xes.insert(x);
        }
    }

    xes.len()
}

struct Ranges {
    inner: Vec<Range<isize>>,
}

impl Ranges {
    fn project(&self) -> Vec<Range<isize>> {
        let mut old_ranges = self.inner.clone();
        old_ranges.sort_by_key(|r| r.start);

        let mut new_ranges = vec![];
        let mut start = isize::MAX;
        let mut end = 0;

        for r in old_ranges {
            if r.start > end + 1 && start != isize::MAX {
                new_ranges.push(start..end);
                start = isize::MAX;
                end = 0;
            }

            if r.start < start {
                start = r.start;
            }

            if r.end > end {
                end = r.end;
            }
        }

        new_ranges.push(start..end);

        new_ranges
    }
}

fn two() -> usize {
    let sensors = read_input().unwrap();

    for y in 0..=4_000_000 {
        let mut ranges = Ranges { inner: vec![] };

        for sensor in &sensors {
            ranges.inner.push(sensor.range_at_y(y));
        }

        if y % 100_000 == 0 {
            dbg!(y);
        }

        let projection = ranges.project();
        if projection.len() > 1 {
            dbg!(projection, y);
            break;
        }
    }

    todo!()
}

fn two_naive() -> usize {
    let sensors = read_input().unwrap();

    let mut row = vec![false; 4_000_001];
    for y in 0..=4_000_000 {
        for sensor in &sensors {
            for x in sensor.range_at_y(y) {
                if x < 0 || x > 4_000_000 {
                    continue;
                }

                row[x as usize] = true;
            }
        }

        for x in 0..row.len() {
            if !row[x] {
                return x * 4000000 + y as usize;
            }
        }

        row.clear();
        row.resize(4_000_001, false);
    }

    unreachable!()
}

struct Sensor {
    location: (isize, isize),
    nearest_beacon: (isize, isize),
}

impl Sensor {
    fn distance_to_beacon(&self) -> isize {
        distance(self.location, self.nearest_beacon)
    }

    fn range_at_y(&self, y: isize) -> Range<isize> {
        let distance_to_beacon = self.distance_to_beacon();
        let distance_to_y = (self.location.1 - y).abs();
        let difference = distance_to_beacon - distance_to_y;
        if difference < 0 {
            return 0..0;
        }
        (self.location.0 - difference)..(self.location.0 + difference)
    }
}

fn distance(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

impl FromStr for Sensor {
    type Err = BoxError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let coords: Vec<isize> = s
            .split("=")
            .skip(1)
            .map(|seg| seg.trim_end_matches(|x: char| !x.is_numeric()))
            .map(|seg| seg.parse::<isize>().unwrap())
            .collect();

        Ok(Sensor {
            location: (coords[0], coords[1]),
            nearest_beacon: (coords[2], coords[3]),
        })
    }
}

fn read_input() -> Result<Vec<Sensor>> {
    let lines = BufReader::new(File::open("input/15.in")?).lines();

    lines.map(|l| l?.parse()).collect()
}

#[test]
fn test_parse_sensor() {
    let sensor: Sensor = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        .parse()
        .unwrap();

    assert_eq!(sensor.location, (2, 18));
    assert_eq!(sensor.nearest_beacon, (-2, 15));
}

#[test]
fn test_project() {
    let mut ranges = Ranges {
        inner: vec![0..10, 1..11, 13..15],
    };

    assert_eq!(ranges.project(), vec![0..11, 13..15]);

    ranges = Ranges {
        inner: vec![10..15, 1..3, 3..10],
    };

    assert_eq!(ranges.project(), vec![1..15]);
}
