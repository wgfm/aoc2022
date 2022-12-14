use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};

use serde_json::{json, Value};

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

#[derive(PartialEq, Eq)]
struct Signal {
    elements: Value,
}

struct SignalPair {
    left: Signal,
    right: Signal,
}

fn one() -> usize {
    let pairs = read_input().unwrap();
    pairs
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .filter(|(_, p)| p.correct_order())
        .map(|(i, _)| i)
        .sum()
}

fn two() -> usize {
    let mut signals = read_input2().unwrap();
    signals.push(Signal {
        elements: json!(vec![vec![2]]),
    });
    signals.push(Signal {
        elements: json!(vec![vec![6]]),
    });
    signals.sort();

    signals
        .iter()
        .enumerate()
        .map(|(i, s)| (i + 1, s))
        .filter(|(_, s)| s.elements == json!(vec![vec![2]]) || s.elements == json!(vec![vec![6]]))
        .map(|(i, _)| i)
        .product()
}

fn cmp(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            let l = l.as_u64().unwrap();
            let r = r.as_u64().unwrap();
            if l == r {
                return Ordering::Equal;
            } else if l < r {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        (Value::Array(l), Value::Array(r)) => {
            let mut compare_results = l.iter().zip(r.iter()).map(|(lv, rv)| cmp(lv, rv));
            match compare_results.find(|r| !r.is_eq()) {
                None => return cmp(&json!(l.len()), &json!(r.len())),
                Some(ord) => ord,
            }
        }
        (l @ Value::Array(_), r @ Value::Number(_)) => cmp(l, &Value::Array(vec![r.clone()])),
        (l @ Value::Number(_), r @ Value::Array(_)) => cmp(&Value::Array(vec![l.clone()]), r),
        _ => unreachable!(),
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp(&self.elements, &other.elements)
    }
}

impl SignalPair {
    fn correct_order(&self) -> bool {
        if let Ordering::Greater = cmp(&self.left.elements, &self.right.elements) {
            false
        } else {
            true
        }
    }
}

fn read_input() -> Result<Vec<SignalPair>> {
    let mut lines = BufReader::new(File::open("input/13.in")?).lines();

    let mut pairs = vec![];
    while let Ok(pair) = parse_signal_pair(&mut lines) {
        pairs.push(pair);
        lines.next();
    }

    Ok(pairs)
}

fn read_input2() -> Result<Vec<Signal>> {
    let mut lines = BufReader::new(File::open("input/13.in")?).lines();

    Ok(lines
        .map(|l| l.unwrap())
        .filter(|l| l != "")
        .map(|l| l.parse().unwrap())
        .collect())
}

fn parse_signal_pair<B: BufRead>(lines: &mut Lines<B>) -> Result<SignalPair> {
    Ok(SignalPair {
        left: lines.next().ok_or("no new line")??.parse()?,
        right: lines.next().ok_or("no new line")??.parse()?,
    })
}

impl FromStr for Signal {
    type Err = BoxError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = serde_json::from_str(s)?;
        Ok(Signal { elements: val })
    }
}

#[test]
fn test_cmp() {
    assert_eq!(cmp(&json!(1), &json!(2)), Ordering::Less);
    assert_eq!(cmp(&json!(2), &json!(2)), Ordering::Equal);
    assert_eq!(cmp(&json!(3), &json!(2)), Ordering::Greater);

    assert_eq!(cmp(&json!(vec![1]), &json!(vec![1])), Ordering::Equal);
    assert_eq!(cmp(&json!(vec![2]), &json!(vec![1])), Ordering::Greater);
    assert_eq!(cmp(&json!(vec![1]), &json!(vec![2])), Ordering::Less);
    assert_eq!(cmp(&json!(vec![1]), &json!(vec![1, 2])), Ordering::Less);
    assert_eq!(cmp(&json!(vec![1, 2]), &json!(vec![1])), Ordering::Greater);
    assert_eq!(
        cmp(&json!(Vec::<usize>::new()), &json!(vec![1])),
        Ordering::Less
    );

    assert_eq!(cmp(&json!(2), &json!(vec![2])), Ordering::Equal);
    assert_eq!(cmp(&json!(2), &json!(vec![1, 3])), Ordering::Greater);
    assert_eq!(cmp(&json!(2), &json!(vec![3, 3])), Ordering::Less);
    assert_eq!(cmp(&json!(2), &json!(vec![3, 3])), Ordering::Less);

    assert_eq!(
        cmp(
            &json!(vec![vec![Vec::<usize>::new()]]),
            &json!(vec![Vec::<usize>::new()])
        ),
        Ordering::Greater
    );

    assert_eq!(
        cmp(&json!(Vec::<usize>::new()), &json!(vec![3])),
        Ordering::Less
    );
}
