use std::vec;

use crate::util::Solution;
use itertools::Itertools;
use tap::prelude::*;

pub const SOLUTION: Solution<'static, u32, u32> = Solution {
    day: 07,
    title: "No Space Left On Device",
    input: include_str!("./main.input"),
    part1,
    part2,
};

#[allow(dead_code)]
#[derive(Debug)]
enum NodeKind<N> {
    FILE,
    DIR { children: Vec<N> },
}

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    size: u32,
    name: String,
    kind: NodeKind<Self>,
}

struct PartialNode {
    size: Option<u32>,
    name: String,
    kind: NodeKind<Self>,
}

impl Node {
    fn flatten_dirs(&self) -> Vec<u32> {
        match &self.kind {
            NodeKind::FILE => vec![],
            NodeKind::DIR { children } => children
                .iter()
                .map(|c| c.flatten_dirs())
                .flatten()
                .collect::<Vec<_>>()
                .tap_mut(|v| v.push(self.size)),
        }
    }
}

impl PartialNode {
    fn root() -> Self {
        PartialNode {
            size: None,
            name: "/".to_owned(),
            kind: NodeKind::DIR { children: vec![] },
        }
    }

    fn insert_node(&mut self, path: &[&str], node: PartialNode) {
        if let Some((current, next)) = path.split_first() {
            match &mut self.kind {
                NodeKind::FILE => panic!("Path {:?} does not exist!", next),
                NodeKind::DIR { children } => {
                    children
                        .iter_mut()
                        .find(|i| i.name == *current)
                        .expect(&format!("No node named '{}'", *current))
                        .insert_node(next, node);
                }
            }
        } else {
            match &mut self.kind {
                NodeKind::FILE => panic!("{} is a file, not a directory.", self.name),
                NodeKind::DIR { children } => children.push(node),
            }
        }
    }

    /// Validates file tree, returning a [Node], which has size: i32 instead of size: Option<i32>
    fn canonicalize(&self) -> Node {
        match &self.kind {
            NodeKind::FILE => Node {
                size: self
                    .size
                    .expect(&format!("Node '{}' is not sized!", self.name.clone())),
                name: self.name.clone(),
                kind: NodeKind::FILE,
            },
            NodeKind::DIR { children } => {
                let children: Vec<_> = children.iter().map(|i| i.canonicalize()).collect();
                let size: u32 = children.iter().map(|i| i.size).sum();

                Node {
                    size,
                    name: self.name.clone(),
                    kind: NodeKind::DIR { children },
                }
            }
        }
    }
}

fn parse(input: &str) -> Node {
    let mut root = PartialNode::root();
    let mut path_segments: Vec<&str> = vec![];
    for l in input.lines() {
        let words: Vec<_> = l.split(" ").collect();
        if words[0] == "$" {
            if words[1] == "cd" {
                match words[2] {
                    ".." => {
                        path_segments.pop();
                    }
                    "/" => path_segments.clear(),
                    destination => path_segments.push(destination),
                }
            }
        } else {
            let name = words[1].to_owned();
            let node_to_add = if words[0] == "dir" {
                PartialNode {
                    size: None,
                    name,
                    kind: NodeKind::DIR { children: vec![] },
                }
            } else {
                PartialNode {
                    size: Some(words[0].parse().expect("Couldn't read file size to u32")),
                    name,
                    kind: NodeKind::FILE,
                }
            };
            root.insert_node(&path_segments, node_to_add);
        }
    }
    root.canonicalize()
}

fn part1(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .flatten_dirs()
            .into_iter()
            .filter(|size| *size <= 100_000)
            .sum(),
    )
}

fn part2(input: &str) -> Option<u32> {
    let root = parse(input);
    let capacity = 70_000_000;
    let usage = root.size;
    let to_free_up = 30_000_000 - (capacity - usage);

    root.flatten_dirs()
        .into_iter()
        .sorted()
        .find(|s| *s >= to_free_up)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = include_str!("./sample.input");

    #[test]
    fn sample_1() {
        let expected = 95437;
        debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
    }

    #[test]
    fn sample_2() {
        let expected = 24933642;
        debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected)
    }
}
