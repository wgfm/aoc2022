use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::util::Result;

#[test]
fn test_one() {
    eprintln!("{}", one());
    assert!(true);
}

#[test]
fn test_two() {
    let forest = read_input().unwrap();
    eprintln!("{}", two(&forest));
    assert!(false);
}

#[test]
fn test_two_testdata() {
    let r = "30373
25512
65332
33549
35390
";

    let mut forest = vec![];
    for line in r.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch as i8 - '0' as i8);
        }

        forest.push(row);
    }

    assert_eq!(two(&forest), 8);
}

fn one() -> usize {
    let forest = read_input().unwrap();
    let mut visible_trees = 0;
    for (y, row) in forest.iter().enumerate() {
        'trees: for (x, tree) in row.iter().enumerate() {
            let left = &row[..x].iter().max().unwrap_or(&-1) < &tree;
            let right = &row[x + 1..].iter().max().unwrap_or(&-1) < &tree;
            let top = &forest[..y].iter().map(|r| r[x]).max().unwrap_or(-1) < &tree;
            let bottom = &forest[y + 1..].iter().map(|r| r[x]).max().unwrap_or(-1) < &tree;

            if left || right || top || bottom {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn two(forest: &Vec<Vec<i8>>) -> usize {
    let mut max_score = 0;
    for (y, row) in forest.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let score = scenic_score(&forest, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn scenic_score(forest: &Vec<Vec<i8>>, x: usize, y: usize) -> usize {
    let tree = forest[y][x];
    //left
    let mut received_left = false;
    let left = forest[y][..x]
        .iter()
        .rev()
        .take_while(move |t| {
            if received_left {
                return false;
            }
            if **t >= tree {
                received_left = true;
            }

            true
        })
        .count() as usize;

    let mut received_right = false;
    let right = forest[y][x + 1..]
        .iter()
        .take_while(move |t| {
            if received_right {
                return false;
            }
            if **t >= tree {
                received_right = true;
            }

            true
        })
        .count() as usize;

    let mut received_top = false;
    let top = forest[..y]
        .iter()
        .rev()
        .map(|r| r[x])
        .take_while(move |t| {
            if received_top {
                return false;
            }
            if *t >= tree {
                received_top = true;
            }

            true
        })
        .count() as usize;

    let mut received_bottom = false;
    let bottom = forest[y + 1..]
        .iter()
        .map(|r| r[x])
        .take_while(move |t| {
            if received_bottom {
                return false;
            }

            if *t >= tree {
                received_bottom = true;
            }

            true
        })
        .count() as usize;

    let score = left * right * bottom * top;
    if score == 9 {
        dbg!((x, y));
        dbg!((top, left, bottom, right));
    }

    score
}

fn read_input() -> Result<Vec<Vec<i8>>> {
    let r = BufReader::new(File::open("input/8.in")?);
    let mut result = vec![];

    for line in r.lines() {
        let mut row = vec![];
        for ch in line?.chars() {
            row.push(ch as i8 - '0' as i8);
        }

        result.push(row);
    }

    Ok(result)
}
