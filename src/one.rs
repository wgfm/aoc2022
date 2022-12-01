use std::{
    cmp::Reverse,
    io::{self, BufRead},
};

use super::util::*;

fn readInput() -> Result<Vec<Vec<u32>>, BoxError> {
    let f = std::fs::File::open("./input/1.in")?;

    let lines = io::BufReader::new(f).lines();

    let mut result = vec![vec![]];
    for line in lines {
        let line = line?;
        if line == "" {
            result.push(vec![]);
            continue;
        }

        result.last_mut().unwrap().push(line.parse()?);
    }

    Ok(result)
}

fn one() -> Result<u32, BoxError> {
    let input = readInput()?;

    let counts: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();

    let mut max = 0;
    let mut max_i = 0;
    for (i, count) in counts.iter().enumerate() {
        if *count > max {
            max = *count;
            max_i = i;
        }
    }

    Ok(max)
}

fn two() -> Result<u32, BoxError> {
    let input = readInput()?;

    let mut counts: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    counts.sort_by_key(|c| Reverse(*c));

    let total: u32 = counts.iter().take(3).sum();
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        eprintln!("{}", one().unwrap());
        assert!(true);
    }

    #[test]
    fn test_two() {
        eprintln!("{}", two().unwrap());
        assert!(false);
    }
}
