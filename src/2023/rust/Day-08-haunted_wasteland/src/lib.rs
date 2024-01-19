#![allow(dead_code, non_snake_case)]

use num::integer::lcm;
use std::collections::HashMap;

enum Movement {
    Left = 0,
    Right = 1,
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

const TARGET: &str = "Z";
const INIT: &str = "A";

fn count_steps(input: &str) -> usize {
    //
    // E.g.
    // LLR
    //
    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)
    let mut splits = input.split("\n").filter(|s| !s.is_empty());
    let mut node_map = HashMap::new();
    let mut current_nodes = Vec::new();

    let instructions = splits.next().unwrap();
    splits.for_each(|s| {
        let input = &s[0..=2];
        let left = &s[7..=9];
        let right = &s[12..=14];

        let n = Node { left, right };
        node_map.insert(input, n);

        if input.ends_with(INIT) {
            current_nodes.push(input);
        }
    });

    let cycle = instructions
        .chars()
        .map(|c| match c {
            'L' => Movement::Left,
            'R' => Movement::Right,
            _ => panic!("Unknown movement"),
        })
        .cycle();

    let mut cycle_numbers = Vec::with_capacity(instructions.len());

    for current_node in current_nodes {
        let mut cycle_number = 0;
        let mut current_node = current_node;
        for movement in cycle.clone() {
            cycle_number += 1;

            let node = node_map.get(current_node).unwrap();

            current_node = match movement {
                Movement::Left => node.left,
                Movement::Right => node.right,
            };

            if current_node.ends_with(TARGET) {
                cycle_numbers.push(cycle_number);
                break;
            }
        }
    }

    let count = cycle_numbers
        .into_iter()
        .reduce(|acc, next| lcm(acc, next))
        .unwrap();

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    // pub mod part1 {

    //     use super::*;

    //     #[test]
    //     fn test_demo_input() {
    //         if !is_demo_mode() {
    //             return;
    //         }

    //         let input = get_input();
    //         let count = count_steps(&input);

    //         assert_eq!(count, 6);
    //     }

    //     #[test]
    //     fn test_input() {
    //         if is_demo_mode() {
    //             return;
    //         }

    //         let input = get_input();
    //         let count = count_steps(&input);

    //         println!("Answer pt1: {}", count);
    //     }
    // }

    pub mod part2 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();
            let count = count_steps(&input);

            assert_eq!(count, 6);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let count = count_steps(&input);

            println!("Answer pt2: {}", count);
        }
    }
}
