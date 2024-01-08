#![allow(dead_code, non_snake_case)]

use std::ops::RangeInclusive;

#[derive(Debug)]
struct Race {
    record_distance: u32,
    time_limit_ms: u32,
}

impl Race {
    fn new(record_distance: u32, time_limit_ms: u32) -> Self {
        Self {
            record_distance,
            time_limit_ms,
        }
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    // Parses input like this:
    // Time:      7  15   30
    // Distance:  9  40  200
    //
    // Where
    // Time:      tl1 tl2 tl3
    // Distance:  rd1 rd2 rd3

    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|str| str.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|str| str.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    time.iter()
        .zip(distance)
        .map(|(t, d)| Race::new(d, *t))
        .collect()
}

fn compute_record_range(record_distance: u32, time_limit: u32) -> RangeInclusive<u32> {
    let R: f32 = record_distance as f32;
    let T: f32 = time_limit as f32;

    let a = T / 2.0;
    let b = (-4.0 * R + T.powi(2)).sqrt() / 2.0;

    let h1 = a + b;
    let h2 = a - b;

    h1.ceil() as u32..=h2.floor() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    pub mod part1 {

        const CORRECT_ANSWERS: [(u32, u32); 3] = [(9, 4), (15, 8), (30, 9)];
        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();

            assert_eq!(input, "hey");
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();

            println!("Answer pt1: {}", input);
        }
    }

    pub mod part2 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();

            assert_eq!(input, "hey");
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();

            println!("Answer pt2: {}", input);
        }
    }
}
