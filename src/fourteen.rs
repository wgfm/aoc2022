use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::util::{BoxError, Result};

#[derive(Clone, Copy, Debug)]
enum Element {
    Air,
    Stone,
    Sand,
}

const SANDY_ORIGIN: (usize, usize) = (500, 0);

//#[test]
fn test_one() {
    eprintln!("{}", one());
    assert!(true);
}

#[test]
fn test_two() {
    eprintln!("{}", two());
    assert!(false);
}

fn two() -> usize {
    let mut cave = read_input().unwrap();
    cave.add_floor();
    cave.pour_sand();

    cave.map
        .iter()
        .flat_map(|row| row.iter().filter(|e| matches!(e, Element::Sand)))
        .count()
}

fn one() -> usize {
    let mut cave = read_input().unwrap();
    cave.pour_sand();
    plot(&cave);

    cave.map
        .iter()
        .flat_map(|row| row.iter().filter(|e| matches!(e, Element::Sand)))
        .count()
}

#[derive(Debug)]
struct Cave {
    map: Vec<Vec<Element>>,
    max_x: usize,
    max_y: usize,
}

fn plot(c: &Cave) {
    for row in &c.map {
        for elem in &row[450..600] {
            match elem {
                Element::Air => print!("."),
                Element::Stone => print!("#"),
                Element::Sand => print!("o"),
            }
        }
        println!();
    }
}

impl Cave {
    fn from_structures(structures: &[Structure]) -> Self {
        let mut max_x = 1000;
        let mut max_y = 0;

        for st in structures {
            for corner in &st.corners {
                max_x = max(max_x, corner.0);
                max_y = max(max_y, corner.1);
            }
        }

        let mut map = vec![vec![Element::Air; max_x]; max_y + 5];

        for st in structures {
            for line in st.corners.windows(2) {
                let (x1, y1) = line[0];
                let (x2, y2) = line[1];

                for y in min(y1, y2)..=max(y1, y2) {
                    for x in min(x1, x2)..=max(x1, x2) {
                        map[y][x] = Element::Stone;
                    }
                }
            }
        }

        Cave { map, max_x, max_y }
    }

    fn add_floor(&mut self) {
        for elem in &mut self.map[self.max_y + 2] {
            *elem = Element::Stone;
        }
    }

    fn check(&self, at: (usize, usize)) -> Element {
        self.map[at.1][at.0]
    }

    fn pour_sand(&mut self) {
        while let Some(_) = self.drop_sand() {}
    }

    fn drop_sand(&mut self) -> Option<()> {
        let mut sand_location = SANDY_ORIGIN;

        while matches!(self.check(sand_location), Element::Air | Element::Sand) {
            let (x, y) = sand_location;

            if y > self.max_y + 2 {
                return None;
            }

            if let Element::Air = self.check((x, y + 1)) {
                sand_location = (x, y + 1);
                continue;
            }

            if let Element::Air = self.check((x - 1, y + 1)) {
                sand_location = (x - 1, y + 1);
                continue;
            }

            if let Element::Air = self.check((x + 1, y + 1)) {
                sand_location = (x + 1, y + 1);
                continue;
            }

            break;
        }

        self.map[sand_location.1][sand_location.0] = Element::Sand;
        if sand_location == SANDY_ORIGIN {
            return None;
        }
        Some(())
    }
}

struct Structure {
    corners: Vec<(usize, usize)>,
}

impl FromStr for Structure {
    type Err = BoxError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let corners: Vec<(usize, usize)> = s
            .split(" -> ")
            .map(|p| p.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();

        Ok(Structure { corners })
    }
}

fn read_input() -> Result<Cave> {
    let lines = BufReader::new(File::open("input/14.in")?).lines();
    let structures: Vec<Structure> = lines
        .map(|l| l?.parse())
        .collect::<Result<Vec<Structure>>>()
        .unwrap();

    Ok(Cave::from_structures(&structures))
}
