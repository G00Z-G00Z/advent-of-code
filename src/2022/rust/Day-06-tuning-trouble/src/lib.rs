use std::collections::HashSet;

pub fn detect_sub_routine(signal: &str) -> Option<u32> {
    for window in signal.chars().enumerate().collect::<Vec<_>>().windows(4) {
        let place = window[3].0 + 1;
        let (a, b, c, d) = (window[0].1, window[1].1, window[2].1, window[3].1);
        let set = HashSet::from([a, b, c, d]);
        if set.len() == 4 {
            return Some(place as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    #[test]
    fn prueba_demo() {
        if !is_demo_mode() {
            println!("Demo mode is not enabled");
            return;
        }

        let input = get_input();

        let result = detect_sub_routine(&input).expect("Test case with input must return a number");

        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_1() {
        if is_demo_mode() {
            println!("Demo mode is enabled");
            return;
        }

        let input = get_input();

        let result = detect_sub_routine(&input).expect("Test case with input must return a number");

        println!("Result pt1: {}", result);
    }

    #[test]
    fn prueba_demo_2() {
        let trials = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for t in trials.iter() {
            let result =
                detect_sub_routine(t.0).expect("Test case with input must return a number");
            assert_eq!(result, t.1);
        }
    }
}
