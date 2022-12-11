use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::util::{BoxError, Result};

#[test]
fn test_one() {
    eprintln!("{}", one());
    assert!(true);
}

#[test]
fn test_two() {
    two();
    assert!(false);
}

fn two() {
    let instructions = read_input().unwrap();
    let mut sprite = instructions
        .into_iter()
        .map(|inst| inst.iter())
        .flatten()
        .scan((1, 1), |(during, after), elem| {
            *during = *after;
            *after += elem;

            Some((*during, *after))
        })
        .enumerate();

    while let Some((i, (during, _))) = sprite.next() {
        let i = i as isize % 40;
        if i == 0 {
            println!();
        }
        if during == i || during - 1 == i || during + 1 == i {
            print!("#");
        } else {
            print!(" ");
        }
    }
}

fn one() -> isize {
    let instructions = read_input().unwrap();
    instructions
        .into_iter()
        .map(|inst| inst.iter())
        .flatten()
        .scan((1, 1), |(during, after), elem| {
            *during = *after;
            *after += elem;

            Some((*during, *after))
        })
        .enumerate()
        .map(|(i, elem)| (i + 1, elem)) // Cycles start at 1
        .map(|(i, (during, after))| dbg!(i, (during, after)))
        .filter(|(i, _)| (i + 2) % 40 == 0)
        .take(6)
        .map(|(i, (during, _))| (i as isize) * during)
        .sum()
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Noop,
    AddX(isize),
}

impl Instruction {
    fn iter(&self) -> InstructionIterator {
        InstructionIterator {
            inner: *self,
            cycles: 0,
        }
    }
}

struct InstructionIterator {
    inner: Instruction,
    cycles: usize,
}

impl Iterator for InstructionIterator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.inner {
            Instruction::Noop => {
                if self.cycles == 1 {
                    None
                } else {
                    Some(0)
                }
            }
            Instruction::AddX(x) => match self.cycles {
                2 => None,
                1 => Some(x),
                0 => Some(0),
                _ => unreachable!(),
            },
        };
        self.cycles += 1;

        result
    }
}

impl FromStr for Instruction {
    type Err = BoxError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        let (_, amount) = s.split_once(' ').unwrap();
        let amount: isize = amount.parse()?;

        Ok(Instruction::AddX(amount))
    }
}

fn read_input() -> Result<Vec<Instruction>> {
    let lines = BufReader::new(File::open("input/10.in")?).lines();
    lines.map(|l| l?.parse()).collect()
}
