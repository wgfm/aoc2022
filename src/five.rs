use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::util::{BoxError, Result};

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

struct Warehouse {
    crates: Vec<Vec<char>>,
}

impl Warehouse {
    fn execute9000(&mut self, instruction: Instruction) {
        for _ in 0..instruction.count {
            let ch = self.crates[instruction.from - 1].pop().unwrap();
            self.crates[instruction.to - 1].push(ch);
        }
    }

    fn execute9001(&mut self, instruction: Instruction) {
        let (new_from, to_move) = {
            let from = &self.crates[instruction.from - 1];
            let at = from.len() - instruction.count;
            let (new_from, to_move) = from.split_at(at);
            (new_from.to_vec(), to_move.to_vec())
        };

        self.crates[instruction.to - 1].extend(to_move);
        self.crates[instruction.from - 1] = new_from;
    }
}

fn one() -> String {
    let (mut warehouse, instructions) = read_input().unwrap();

    for instr in instructions {
        warehouse.execute9000(instr);
    }

    let mut result = String::new();
    for mut stack in warehouse.crates {
        result.push(stack.pop().unwrap());
    }

    result
}

fn two() -> String {
    let (mut warehouse, instructions) = read_input().unwrap();

    for instr in instructions {
        warehouse.execute9001(instr);
    }

    let mut result = String::new();
    for mut stack in warehouse.crates {
        result.push(stack.pop().unwrap());
    }

    result
}

impl FromStr for Instruction {
    type Err = BoxError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut strs = s.split(' ');
        strs.next(); // move
        let count: usize = strs.next().unwrap().parse()?;
        strs.next(); // from
        let from: usize = strs.next().unwrap().parse()?;
        strs.next(); // to
        let to = strs.next().unwrap().parse()?;

        Ok(Self { count, from, to })
    }
}

fn read_input() -> Result<(Warehouse, Vec<Instruction>)> {
    let mut lines = BufReader::new(File::open("input/5.in")?).lines();

    let crate_lines = lines
        .by_ref()
        .take_while(|l| !l.as_ref().unwrap().contains("1"));

    let mut stacks = Vec::new();
    for _ in 0..9 {
        stacks.push(vec![]);
    }

    for line in crate_lines {
        for (i, ch) in line?.chars().enumerate() {
            if ch.is_alphabetic() {
                stacks[i / 4].push(ch)
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let instructions = lines
        .skip_while(|l| !l.as_ref().unwrap().starts_with("move"))
        .map(|l| Instruction::from_str(&l.unwrap()))
        .map(|i| i.unwrap())
        .collect();

    Ok((Warehouse { crates: stacks }, instructions))
}

#[test]
fn test_one() {
    eprintln!("{}", one());
    assert!(true);
}

#[test]
fn test_two() {
    eprintln!("{}", two());
    assert!(false);
}
