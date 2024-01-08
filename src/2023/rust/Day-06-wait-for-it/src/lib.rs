#![allow(dead_code, non_snake_case)]

use std::ops::RangeInclusive;

#[derive(Debug)]
struct Race {
    record_distance: u64,
    time_limit_ms: u64,
}

impl Race {
    fn new(record_distance: u64, time_limit_ms: u64) -> Self {
        Self {
            record_distance,
            time_limit_ms,
        }
    }
}

fn parse_input_2(input: &str) -> Vec<Race> {
    // Parses input like this:
    // Time:      7  15   30
    // Distance:  9  40  200
    //
    // Where (ignore spaces)
    // Time:      time
    // Distance:  distance

    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(|char| char.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .chars()
        .filter(|char| char.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    vec![Race::new(distance, time)]
}
fn parse_input_1(input: &str) -> Vec<Race> {
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
        .filter_map(|str| str.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|str| str.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    time.iter()
        .zip(distance)
        .map(|(t, d)| Race::new(d, *t))
        .collect()
}

// OJO: Tiene que ser mayor o igual al record
fn compute_record_range(record_distance: u64, time_limit: u64) -> RangeInclusive<u64> {
    let R: f64 = record_distance as f64;
    let T: f64 = time_limit as f64;

    let a = T / 2.0;
    let b = (-4.0 * R + T.powi(2)).sqrt() / 2.0;

    let h1 = (a - b).ceil();
    let h2 = (a + b).floor();

    // Check for equal time
    let compute_distance = |ht: f64| T * ht - ht.powi(2);

    // println!("h1: {}, h2: {}", h1, h2);
    // println!("d1: {}, d2: {}", compute_distance(h1), compute_distance(h2));

    let h1 = if compute_distance(h1) > R {
        h1
    } else {
        h1 + 1.0
    } as u64;

    let h2 = if compute_distance(h2) > R {
        h2
    } else {
        h2 - 1.0
    } as u64;

    h1..=h2
}

fn count_ways(range: RangeInclusive<u64>) -> u64 {
    *range.end() - *range.start() + 1
}

fn part_1(input: &str) -> u64 {
    let races = parse_input_1(&input);
    let possible_scores = races
        .iter()
        .map(|race| compute_record_range(race.record_distance, race.time_limit_ms))
        .map(|r| count_ways(r))
        .collect::<Vec<u64>>();

    possible_scores.iter().fold(1, |acc, x| acc * x)
}

fn part_2(input: &str) -> u64 {
    let races = parse_input_2(&input);
    let possible_scores = races
        .iter()
        .map(|race| compute_record_range(race.record_distance, race.time_limit_ms))
        .map(|r| count_ways(r))
        .collect::<Vec<u64>>();

    possible_scores.iter().fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    pub mod part1 {

        const CORRECT_ANSWERS: [(u64, u64); 3] = [(9, 4), (15, 8), (30, 9)];
        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();
            let ans = part_1(&input);
            // let races = parse_input(&input);
            // let possible_scores = races
            //     .iter()
            //     .map(|race| compute_record_range(race.record_distance, race.time_limit_ms).count())
            //     .collect::<Vec<_>>();

            // // println!("{:?}", possible_scores);

            // for (answer, race) in CORRECT_ANSWERS.iter().zip(possible_scores.iter()) {
            //     assert_eq!(answer.1, *race as u64, "Races do not match")
            // }

            assert_eq!(288, ans);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let ans = part_1(&input);
            println!("Answer pt1: {}", ans);
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
            let ans = part_2(&input);

            assert_eq!(71503, ans);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let ans = part_2(&input);

            println!("Answer pt2: {}", ans);
        }
    }
}
