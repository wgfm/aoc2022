use crate::util::{BoxError, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

struct Pair {
    left: RangeInclusive<usize>,
    right: RangeInclusive<usize>,
}

impl Pair {
    fn one_encompasses_other(&self) -> bool {
        self.left.clone().all(|c| self.right.contains(&c))
            || self.right.clone().all(|c| self.left.contains(&c))
    }

    fn one_overlaps_other(&self) -> bool {
        self.left.clone().any(|c| self.right.contains(&c))
    }
}

fn one() -> usize {
    let pairs = read_input().unwrap();

    pairs
        .iter()
        .filter(|pair| pair.one_encompasses_other())
        .count()
}

fn two() -> usize {
    let pairs = read_input().unwrap();

    pairs
        .iter()
        .filter(|pair| pair.one_overlaps_other())
        .count()
}

impl FromStr for Pair {
    type Err = BoxError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').unwrap();
        Ok(Pair {
            left: str_to_range(left),
            right: str_to_range(right),
        })
    }
}

fn read_input() -> Result<Vec<Pair>> {
    let r = BufReader::new(File::open("input/4.in")?);

    let mut pairs = vec![];
    for line in r.lines() {
        pairs.push(Pair::from_str(&line?)?);
    }

    Ok(pairs)
}

fn str_to_range(s: &str) -> RangeInclusive<usize> {
    let (from, to) = s.split_once('-').unwrap();

    let from = from.parse().unwrap();
    let to = to.parse().unwrap();

    from..=to
}

#[test]
fn test_one() {
    eprintln!("{}", one());
    assert!(true);
}

#[test]
fn test_two() {
    eprintln!("{}", two());
    assert!(true);
}
