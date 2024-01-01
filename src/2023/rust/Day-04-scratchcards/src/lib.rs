use std::collections::HashSet;

type ScratchNumbers = HashSet<u8>;

trait ScratchNumberFromNumbersSplit {
    fn from_numbers_split(numbers_split: &str) -> Self;
}

impl ScratchNumberFromNumbersSplit for ScratchNumbers {
    fn from_numbers_split(numbers_split: &str) -> Self {
        numbers_split
            .split(" ")
            .filter_map(|number| number.parse::<u8>().ok())
            .collect::<ScratchNumbers>()
    }
}

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: ScratchNumbers,
    scratch_numbers: ScratchNumbers,
}

impl ScratchCard {
    fn new(winning_numbers: ScratchNumbers, scratch_numbers: ScratchNumbers) -> Self {
        Self {
            winning_numbers,
            scratch_numbers,
        }
    }

    fn points(&self) -> u32 {
        let matches = self
            .winning_numbers
            .intersection(&self.scratch_numbers)
            .count() as u32;

        if matches == 0 {
            0
        } else {
            2_u32.pow(matches - 1)
        }
    }
}

#[derive(Debug)]
struct ScratchCardList {
    cards: Vec<ScratchCard>,
    copies: Vec<usize>,
}

impl ScratchCardList {
    fn new(cards: Vec<ScratchCard>) -> Self {
        Self {
            copies: vec![1; cards.len()],
            cards,
        }
    }
}

fn parse_input(input: &str) -> Vec<ScratchCard> {
    input
        .lines()
        .map(|card| {
            let numbers_part = card.split(":").skip(1).next().unwrap().trim();
            let mut numbers_splits = numbers_part.split("|").map(|part| part.trim());

            let winning_part = numbers_splits.next().unwrap();
            let selected_numbers = numbers_splits.next().unwrap();

            let winning_numbers = ScratchNumbers::from_numbers_split(winning_part);
            let scratch_numbers = ScratchNumbers::from_numbers_split(selected_numbers);

            ScratchCard::new(winning_numbers, scratch_numbers)
        })
        .collect::<Vec<ScratchCard>>()
}

fn parse_scratchcard_copies(scratchcard_list: &ScratchCardList) -> usize {
    let mut copies = scratchcard_list.copies.clone();

    for (card_idx, cards) in scratchcard_list.cards.iter().enumerate() {
        let points = cards.points();
        if points == 0 {
            continue;
        }

        let matches = (points.ilog2() + 1) as usize;
        let current_copy = copies.get(card_idx).unwrap().clone();

        for idx in card_idx + 1..card_idx + 1 + matches {
            let no_copy = copies.get_mut(idx);

            if let Some(copy) = no_copy {
                *copy += 1 * current_copy;
            }
        }
    }

    copies.iter().sum()
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

    //         let cards = parse_input(&input);
    //         // print!("{:#?}", cards);

    //         let mut sum = 0;
    //         for card in cards {
    //             let points = card.points();
    //             println!("Card points: {}", points);
    //             sum += points;
    //         }

    //         assert_eq!(13, sum);
    //     }

    //     #[test]
    //     fn test_input() {
    //         if is_demo_mode() {
    //             return;
    //         }

    //         let input = get_input();
    //         let sum = parse_input(&input)
    //             .iter()
    //             .map(|card| card.points())
    //             .sum::<u32>();

    //         println!("Answer pt1: {}", sum);
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
            let cards = parse_input(&input);
            let card_list = ScratchCardList::new(cards);
            let copies = parse_scratchcard_copies(&card_list);

            assert_eq!(30, copies);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let cards = parse_input(&input);
            let card_list = ScratchCardList::new(cards);
            let copies = parse_scratchcard_copies(&card_list);

            println!("Answer pt2: {}", copies);
        }
    }
}
