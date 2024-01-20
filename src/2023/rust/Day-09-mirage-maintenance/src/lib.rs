#![allow(dead_code, non_snake_case)]

type Sequence = Vec<i32>;

pub fn parse_sequences(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Sequence>()
        })
        .collect::<Vec<Sequence>>()
}

pub fn extrapolate_next_number(sequence: &Sequence) -> i32 {
    let mut sequences = vec![sequence.clone()];

    // Generate more sequences

    while !sequences.last().unwrap().iter().all(|n| *n == 0) {
        let mut new_sequence = Vec::with_capacity(sequence.len() - 1);

        for window in sequences.last().unwrap().windows(2) {
            new_sequence.push(window[1] - window[0]);
        }

        sequences.push(new_sequence);
    }

    // Extrapolate next number

    // b - a = c
    // b - c = a

    sequences.iter().rev().skip(1).fold(0, |acc, next| {
        let first = next[0];
        first - acc
    })
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
    //         let sequences = parse_sequences(&input);
    //         let answer = sequences
    //             .iter()
    //             .map(|sequence| extrapolate_next_number(sequence))
    //             .sum::<i32>();

    //         assert_eq!(114, answer);
    //     }

    //     #[test]
    //     fn test_input() {
    //         if is_demo_mode() {
    //             return;
    //         }

    //         let input = get_input();
    //         let sequences = parse_sequences(&input);
    //         let answer = sequences
    //             .iter()
    //             .map(|sequence| extrapolate_next_number(sequence))
    //             .sum::<i32>();

    //         println!("Answer pt1: {}", answer);
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
            let sequences = parse_sequences(&input);
            let answer = sequences
                .iter()
                .map(|sequence| extrapolate_next_number(sequence))
                .collect::<Vec<i32>>();

            println!("answer: {:?}", answer);

            let answer = answer.iter().sum::<i32>();

            assert_eq!(2, answer);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let sequences = parse_sequences(&input);
            let answer = sequences
                .iter()
                .map(|sequence| extrapolate_next_number(sequence))
                .sum::<i32>();

            println!("Answer pt2: {}", answer);
        }
    }
}
