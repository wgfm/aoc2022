use std::{
    io::{self, BufRead},
    str::FromStr,
};

use crate::util::BoxError;

struct Game {
    them: Hand,
    you: Hand,
}

impl Game {
    fn play(&self) -> usize {
        match self.you.value() - self.them.value() {
            0 => 3,
            1 => 6,
            -2 => 6,
            _ => 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn innate_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn value(&self) -> isize {
        self.innate_score() as isize
    }
}

impl FromStr for Game {
    type Err = String;

    // impl two
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chs = s.chars();
        let them = match chs.next().unwrap() {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            h => return Err(format!("unexpected hand: {} ", h)),
        };

        chs.next();

        let you = match (them, chs.next().unwrap()) {
            (Hand::Rock, 'X') => Hand::Scissors,
            (Hand::Rock, 'Y') => Hand::Rock,
            (Hand::Rock, 'Z') => Hand::Paper,
            (Hand::Paper, 'X') => Hand::Rock,
            (Hand::Paper, 'Y') => Hand::Paper,
            (Hand::Paper, 'Z') => Hand::Scissors,
            (Hand::Scissors, 'X') => Hand::Paper,
            (Hand::Scissors, 'Y') => Hand::Scissors,
            (Hand::Scissors, 'Z') => Hand::Rock,
            (_, h) => return Err(format!("unexpected hand: {}", h)),
        };

        Ok(Game { you, them })
    }

    // impl one
    /*
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chs = s.chars();
        let them = match chs.next().unwrap() {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            h => return Err(format!("unexpected hand: {} ", h)),
        };

        chs.next();

        let you = match chs.next().unwrap() {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            h => return Err(format!("unexpected hand: {} ", h)),
        };

        Ok(Game { them, you })
    }
    */
}

fn one() -> usize {
    let games = read_input().unwrap();

    games
        .into_iter()
        .map(|game| game.play() + game.you.innate_score())
        .sum()
}

fn read_input() -> Result<Vec<Game>, BoxError> {
    let f = std::fs::File::open("./input/2.in")?;

    let lines = io::BufReader::new(f).lines();

    Ok(lines.map(|l| l.unwrap().parse().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        eprintln!("{}", one());

        assert!(true);
    }
}
