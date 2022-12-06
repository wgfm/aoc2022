use std::{collections::HashSet, fs};

use crate::util::{BoxError, Result};

fn one() -> usize {
    solve(4)
}

fn two() -> usize {
    solve(14)
}

fn solve(marker_size: usize) -> usize {
    let chars = read_input().unwrap();

    let mut result = 0;
    for (i, seq) in chars.windows(marker_size).enumerate() {
        let set: HashSet<char> = seq.clone().iter().map(|c| *c).collect();

        if set.len() == marker_size {
            result = i + marker_size;
            break;
        }
    }

    result
}

fn read_input() -> Result<Vec<char>> {
    let s = fs::read_to_string("input/6.in")?;
    Ok(s.chars().collect())
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
