use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    num::ParseIntError,
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

fn two() -> usize {
    let mut monkeys = read_input().unwrap();

    let worry_level_modifier: usize = monkeys.iter().map(|m| m.test).product();
    solve(&mut monkeys, 10_000, move |w| w % worry_level_modifier)
}

fn one() -> usize {
    let mut monkeys = read_input().unwrap();
    solve(&mut monkeys, 20, |w| w / 3)
}

fn solve<F: Fn(usize) -> usize>(monkeys: &mut Vec<Monkey>, rounds: usize, zennify: F) -> usize {
    let mut inspections: Vec<usize> = std::iter::repeat(0).take(monkeys.len()).collect();

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let items = &monkeys[m].items;

            let mut truths = vec![];
            let mut falses = vec![];

            for item in items {
                inspections[m] += 1;

                let mut worry_level = match monkeys[m].operation {
                    Operation::Add(x) => item + x,
                    Operation::Mul(x) => item * x,
                    Operation::Sqr => item * item,
                };

                worry_level = zennify(worry_level);
                if worry_level % monkeys[m].test == 0 {
                    truths.push(worry_level);
                } else {
                    falses.push(worry_level);
                }
            }

            let if_true = monkeys[m].if_true;
            let if_false = monkeys[m].if_false;

            monkeys[if_true].items.append(&mut truths);
            monkeys[if_false].items.append(&mut falses);
            monkeys[m].items.clear();
        }
    }

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
}

fn read_input() -> Result<Vec<Monkey>> {
    let mut lines = BufReader::new(File::open("input/11.in")?).lines();

    let mut monkeys = vec![];
    while let Some(monkey) = parse_monkey(&mut lines) {
        monkeys.push(monkey);
    }

    Ok(monkeys)
}

enum Operation {
    Add(usize),
    Mul(usize),
    Sqr,
}

fn parse_monkey<B: BufRead>(l: &mut Lines<B>) -> Option<Monkey> {
    let id = l.next()?;

    assert!(id.unwrap().starts_with("Monkey"));

    let items = l.next()?;
    let items: std::result::Result<Vec<usize>, _> = items
        .unwrap()
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|l| l.parse())
        .collect();

    let items = items.unwrap();

    let operation = l.next()?.unwrap();
    let (op, arg) = operation
        .strip_prefix("  Operation: new = old ")
        .unwrap()
        .split_once(" ")
        .unwrap();

    let operation = match (op, arg) {
        ("*", "old") => Operation::Sqr,
        ("*", num) => Operation::Mul(num.parse().unwrap()),
        ("+", num) => Operation::Add(num.parse().unwrap()),
        _ => unreachable!(),
    };

    let test: usize = l
        .next()?
        .unwrap()
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();

    let if_true: usize = l
        .next()?
        .unwrap()
        .strip_prefix("    If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    let if_false: usize = l
        .next()?
        .unwrap()
        .strip_prefix("    If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    l.next();

    Some(Monkey {
        items,
        operation,
        test,
        if_true,
        if_false,
    })
}
