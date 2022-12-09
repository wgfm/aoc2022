use std::{
    cmp::{max, min},
    collections::HashSet,
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
    eprintln!("{}", two());
    assert!(false);
}

struct Rope {
    knots: Vec<(isize, isize)>,
}

impl Rope {
    fn move_head(&mut self, direction: char) {
        match direction {
            'L' => self.knots[0].0 -= 1,
            'R' => self.knots[0].0 += 1,
            'U' => self.knots[0].1 += 1,
            'D' => self.knots[0].1 -= 1,
            _ => unreachable!(),
        }

        self.move_rest();
    }

    fn move_rest(&mut self) {
        for i in 1..self.knots.len() {
            let mut dist_x = self.knots[i - 1].0 - self.knots[i].0;
            let mut dist_y = self.knots[i - 1].1 - self.knots[i].1;

            assert!(dist_x.abs() <= 2);
            assert!(dist_y.abs() <= 2);

            if dist_x.abs() + dist_y.abs() == 3 {
                if dist_x.abs() == 1 {
                    dist_x *= 2;
                }
                if dist_y.abs() == 1 {
                    dist_y *= 2;
                }
            }

            self.knots[i].0 += dist_x / 2;
            self.knots[i].1 += dist_y / 2;
        }
    }

    fn tail(&self) -> (isize, isize) {
        *self.knots.last().unwrap()
    }
}

fn one() -> usize {
    solve(2)
}

fn two() -> usize {
    solve(10)
}

fn solve(n: usize) -> usize {
    let mut tail_visits = HashSet::new();
    tail_visits.insert((0, 0));
    let mut rope = Rope {
        knots: std::iter::repeat((0, 0)).take(n).collect(),
    };

    for mv in read_input().unwrap() {
        for _ in 0..mv.amount {
            rope.move_head(mv.direction);
            tail_visits.insert(rope.tail());
            //plot(rope.head, &tail_visits);
            println!();
        }
    }

    plot(rope.knots[0], &tail_visits);

    tail_visits.len()
}

fn plot(head: (isize, isize), tail_visits: &HashSet<(isize, isize)>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in tail_visits {
        min_x = min(*x, min_x);
        min_y = min(*y, min_y);
        max_x = max(*x, max_x);
        max_y = max(*y, max_y);
    }

    for y in (0..=(max_y - min_y)).rev() {
        for x in 0..=(max_x - min_x) {
            if head == (x + min_x, y + min_y) {
                print!("H");
            } else if tail_visits.contains(&(x + min_x, y + min_y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

struct Move {
    direction: char,
    amount: usize,
}

impl FromStr for Move {
    type Err = BoxError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (direction, amount) = s.split_once(' ').unwrap();
        Ok(Move {
            direction: direction.chars().next().unwrap(),
            amount: amount.parse()?,
        })
    }
}

fn read_input() -> Result<Vec<Move>> {
    let r = BufReader::new(File::open("input/9.in")?);

    r.lines().map(|line| line?.parse()).collect()
}

#[test]
fn test_movement_movement() {
    let mut rope = Rope {
        knots: vec![(0, 0), (0, 0)],
    };

    rope.move_head('R');
    assert_eq!(rope.tail(), (0, 0));

    rope.move_head('R');
    assert_eq!(rope.tail(), (1, 0));

    rope.move_head('U');
    assert_eq!(rope.tail(), (1, 0));

    rope.move_head('U');
    assert_eq!(rope.tail(), (2, 1));
}
