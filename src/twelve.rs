use crate::util::Result;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

struct World {
    height_map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[test]
fn one_test() {
    eprintln!("{}", one());
    assert!(true);
}

#[test]
fn two_test() {
    eprintln!("{}", two());
    assert!(false);
}

fn one() -> usize {
    let world = read_input().unwrap();
    shortest_path(&world, world.start)
}

fn two() -> usize {
    let world = read_input().unwrap();

    let mut lowest_positions = vec![];
    for (y, row) in world.height_map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height > 0 {
                continue;
            }

            lowest_positions.push((x, y));
        }
    }

    let mut shortest_paths: Vec<usize> = lowest_positions
        .iter()
        .map(|p| shortest_path(&world, *p))
        .collect();

    shortest_paths.sort();
    shortest_paths[0]
}

fn shortest_path(world: &World, start: (usize, usize)) -> usize {
    let mut scores = vec![vec![usize::MAX; world.height_map[0].len()]; world.height_map.len()];

    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);
    scores[start.1][start.0] = 0;

    while let Some(this) = to_visit.pop_front() {
        let this_score = scores[this.1][this.0];

        if this == world.end {
            break;
        }

        for neighbour in world.neighbours(this) {
            if scores[neighbour.1][neighbour.0] <= this_score + 1 {
                continue;
            }

            scores[neighbour.1][neighbour.0] = this_score + 1;
            to_visit.push_back(neighbour);
        }
    }

    scores[world.end.1][world.end.0]
}

impl World {
    fn neighbours(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbouring_points = vec![(point.0 + 1, point.1), (point.0, point.1 + 1)];

        if point.0 > 0 {
            neighbouring_points.push((point.0 - 1, point.1));
        }

        if point.1 > 0 {
            neighbouring_points.push((point.0, point.1 - 1));
        }

        neighbouring_points
            .into_iter()
            .flat_map(|(x, y)| {
                self.height_map
                    .get(y)
                    .and_then(|r| r.get(x))
                    .and_then(|neighbour_level| {
                        if *neighbour_level <= self.height_map[point.1][point.0] + 1 {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}

fn read_input() -> Result<World> {
    let lines = BufReader::new(File::open("input/12.in")?).lines();

    let mut end: (usize, usize) = (0, 0);
    let mut start: (usize, usize) = (0, 0);

    let mut height_map = vec![];
    for (y, line) in lines.enumerate() {
        let mut row = vec![];
        for (x, ch) in line?.chars().enumerate() {
            let dot = match ch {
                'E' => {
                    end = (x, y);
                    'z' as u8 - 'a' as u8
                }
                'S' => {
                    start = (x, y);
                    0
                }
                ch => ch as u8 - 'a' as u8,
            };

            row.push(dot);
        }
        height_map.push(row);
    }

    assert_ne!(start, (0, 0));
    assert_ne!(end, (0, 0));

    Ok(World {
        height_map,
        start,
        end,
    })
}
