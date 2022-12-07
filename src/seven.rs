use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use crate::util::Result;

#[derive(Debug)]
enum Node {
    Dir(HashMap<String, Node>),
    File(usize),
}

impl Node {
    fn size(&self) -> usize {
        match self {
            Node::Dir(entry) => entry.iter().map(|(_, n)| n.size()).sum(),
            Node::File(size) => *size,
        }
    }
}

fn read_input() -> Result<Node> {
    let r = BufReader::new(File::open("input/7.in")?);

    parse_dir_contents(&mut r.lines())
}

fn parse_dir_contents<B>(lines: &mut Lines<B>) -> Result<Node>
where
    B: BufRead,
{
    let mut entries = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.starts_with("$ cd") {
            let name = &line[5..];
            if name == ".." {
                break;
            }

            entries.insert(name.to_string(), parse_dir_contents(lines)?);
            continue;
        }

        if line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("dir") {
            continue; // handled above
        }

        let (size, name) = line.split_once(' ').unwrap();
        let size: usize = size.parse()?;

        entries.insert(name.to_string(), Node::File(size));
    }

    Ok(Node::Dir(entries))
}

#[test]
fn one() {
    let input = read_input().unwrap();

    eprintln!("{}", solve_one(100_000, &input));

    assert!(true);
}

#[test]
fn two() {
    let input = read_input().unwrap();
    eprintln!("{}", solve_two(&input));
    assert!(false);
}

fn solve_two(node: &Node) -> usize {
    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let space_free = total_space - node.size();

    let to_free_space = needed_space - space_free;

    //    eprintln!("{}", &to_free_space);

    let mut dirs = vec![];
    push_dir_sizes(node, &mut dirs);
    dirs.sort();

    *dirs
        .iter()
        .skip_while(|size| **size < to_free_space)
        .next()
        .unwrap()
}

fn push_dir_sizes(node: &Node, dirs: &mut Vec<usize>) {
    if let Node::Dir(entries) = node {
        dirs.push(node.size());
        for (_, entry) in entries {
            push_dir_sizes(entry, dirs);
        }
    }
}

fn solve_one(max_size: usize, node: &Node) -> usize {
    let mut sum = 0;
    let cur_node_sum = match node {
        Node::File(_) => 0,
        Node::Dir(_) => node.size(),
    };

    if cur_node_sum <= max_size {
        sum += cur_node_sum;
    }

    if let Node::Dir(entries) = node {
        for (_, entry) in entries {
            sum += solve_one(max_size, entry);
        }
    }

    sum
}
