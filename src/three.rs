use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::util::Result;

struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

impl Rucksack {
    fn common_item(&self) -> char {
        let intersection: Vec<char> = self.left.intersection(&self.right).map(|ch| *ch).collect();
        if intersection.len() != 1 {
            panic!("more than one common item");
        }

        intersection[0]
    }

    fn all_items(&self) -> HashSet<char> {
        self.left.union(&self.right).map(|ch| *ch).collect()
    }
}

fn read_input() -> Result<Vec<Rucksack>> {
    let r = BufReader::new(File::open("input/3.in")?);

    let rucksacks = r
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let length = line.len();

            let (left_chars, right_chars) = line.split_at(length / 2);

            let left: HashSet<char> = left_chars.chars().collect();
            let right: HashSet<char> = right_chars.chars().collect();

            Rucksack { left, right }
        })
        .collect();

    Ok(rucksacks)
}

fn priority(ch: char) -> usize {
    if ch.is_lowercase() {
        (ch as u8 - b'a') as usize + 1
    } else {
        (ch as u8 - b'A') as usize + 27
    }
}

fn one() -> Result<usize> {
    let rucksacks = read_input()?;

    let s = rucksacks
        .iter()
        .map(|rucksack| priority(rucksack.common_item()))
        .sum();

    Ok(s)
}

fn two() -> Result<usize> {
    let rucksacks = read_input()?;

    let groups = rucksacks.chunks(3);

    let sum = groups
        .map(|group| {
            let (first, rest) = group.split_first().unwrap();
            let thing = rest
                .iter()
                .map(|rucksack| rucksack.all_items())
                .fold(first.all_items(), |acc, curr| {
                    acc.intersection(&curr).map(|ch| *ch).collect()
                });

            let badge_candidates: Vec<char> = thing.iter().map(|ch| *ch).collect();
            if badge_candidates.len() != 1 {
                panic!("no single badge found");
            }

            badge_candidates[0]
        })
        .map(|item| priority(item))
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('A'), 27);
    }

    #[test]
    fn test_two() {
        eprintln!("{}", two().unwrap());
        assert!(false);
    }

    #[test]
    fn test_one() {
        eprintln!("{}", one().unwrap());
        assert!(true);
    }
}
