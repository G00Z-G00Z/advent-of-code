#![allow(dead_code, non_snake_case)]

use std::cmp::Ordering;
#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
}

pub type CardDeck = [char; 5];

#[derive(Debug)]
struct Hand {
    cards: CardDeck,
    hand_type: HandType,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card = self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{}:{:?}", card, self.hand_type)
    }
}

#[derive(Debug)]
struct HandWithBid {
    hand: Hand,
    bid: usize,
}

impl HandWithBid {
    fn new(hand: Hand, bid: usize) -> HandWithBid {
        HandWithBid { hand, bid }
    }
}

impl Display for HandWithBid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hand, self.bid)
    }
}

pub const CARD_ORDER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn get_card_value(card: char) -> usize {
    // The smaller the index, the higher the value
    CARD_ORDER
        .iter()
        .position(|&c| c == card)
        .expect("Card not found")
}

impl Hand {
    fn new(cards: CardDeck) -> Hand {
        let hand_type = HandType::new(cards);
        Hand { cards, hand_type }
    }

    fn beats(&self, other: &Hand) -> Outcome {
        if self.hand_type.value() == other.hand_type.value() {
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                if get_card_value(*c1) == get_card_value(*c2) {
                    continue;
                }

                return if get_card_value(*c1) < get_card_value(*c2) {
                    Outcome::Win
                } else {
                    Outcome::Loss
                };
            }

            unreachable!("Hands are tied");
        }

        if self.hand_type.value() < other.hand_type.value() {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

use std::collections::HashMap;
use std::fmt::Display;

use HandType::*;

impl HandType {
    fn new(hand: CardDeck) -> HandType {
        let mut hand_counts: HashMap<char, u8> = HashMap::new();

        for card in hand.iter() {
            let count = hand_counts.entry(*card).or_insert(0);
            *count += 1;
        }

        let jokers_count = hand_counts.get(&'J').unwrap_or(&0).clone();

        if jokers_count == 5 {
            return FiveOfAKind;
        }

        // Reset the jokers
        hand_counts.remove(&'J');

        let mut counts: Vec<u8> = hand_counts.values().cloned().collect();
        counts.sort();
        counts.reverse();

        // Add jokers to highest count
        counts[0] += jokers_count;

        match counts.len() {
            1 => FiveOfAKind,
            2 => {
                if counts[0] == 4 {
                    FourOfAKind
                } else {
                    FullHouse
                }
            }
            3 => {
                if counts[0] == 3 {
                    ThreeOfAKind
                } else {
                    TwoPair
                }
            }
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!(
                "Invalid hand {}\nwith counts {:?}",
                hand.iter().collect::<String>(),
                counts
            ),
        }
    }

    fn value(&self) -> usize {
        match self {
            FiveOfAKind => 0,
            FourOfAKind => 1,
            FullHouse => 2,
            ThreeOfAKind => 3,
            TwoPair => 4,
            OnePair => 5,
            HighCard => 6,
        }
    }
}

fn parse_input(input: &str) -> Vec<HandWithBid> {
    // Example inputs:
    // 32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(" ");

            let hand = splits.next().unwrap().chars().collect::<Vec<char>>();
            let hand = Hand::new(hand.try_into().expect("Expected 5 cards"));
            let bid = splits.next().unwrap().parse::<usize>().unwrap();

            HandWithBid::new(hand, bid)
        })
        .collect()
}

fn sort_hands(hands: &mut Vec<HandWithBid>) {
    hands.sort_by(|a, b| {
        let outcome = a.hand.beats(&b.hand);

        match outcome {
            Outcome::Loss => Ordering::Less,
            Outcome::Win => Ordering::Greater,
        }
    })
}

fn part_1(input: &str) -> usize {
    let mut hands = parse_input(input);
    sort_hands(&mut hands);
    for hand in hands.iter().enumerate() {
        println!("{}:{}", hand.0 + 1, hand.1);
    }
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let total = hand.bid * (idx + 1);
            total
        })
        .sum::<usize>()
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
    //         let mut hands = parse_input(&input);

    //         // for hand in hands.iter() {
    //         //     println!("{:?}", hand);
    //         // }

    //         assert_eq!(hands.len(), 5);

    //         assert!(
    //             matches!(hands[0].hand.hand_type, HandType::OnePair),
    //             "{} has {:?}; OnePair do not match",
    //             hands[0].hand.cards.iter().collect::<String>(),
    //             hands[0].hand.hand_type
    //         );
    //         assert!(
    //             matches!(hands[1].hand.hand_type, HandType::ThreeOfAKind),
    //             "{} has {:?}; TwoPair do not match",
    //             hands[1].hand.cards.iter().collect::<String>(),
    //             hands[1].hand.hand_type
    //         );
    //         assert!(
    //             matches!(hands[2].hand.hand_type, HandType::TwoPair),
    //             "{} has {:?}; TwoPair do not match",
    //             hands[2].hand.cards.iter().collect::<String>(),
    //             hands[2].hand.hand_type
    //         );
    //         assert!(
    //             matches!(hands[3].hand.hand_type, HandType::TwoPair),
    //             "{} has {:?}; ThreeOfAKind do not match",
    //             hands[3].hand.cards.iter().collect::<String>(),
    //             hands[3].hand.hand_type
    //         );
    //         assert!(
    //             matches!(hands[4].hand.hand_type, HandType::ThreeOfAKind),
    //             "{} has {:?}; ThreeOfAKind do not match",
    //             hands[4].hand.cards.iter().collect::<String>(),
    //             hands[4].hand.hand_type
    //         );

    //         let c1 = Hand::new("KK677".chars().collect::<Vec<char>>().try_into().unwrap());
    //         let c2 = Hand::new("KTJJT".chars().collect::<Vec<char>>().try_into().unwrap());

    //         assert!(
    //             matches!(c1.beats(&c2), Outcome::Win),
    //             "{} should beat {}",
    //             c1.cards.iter().collect::<String>(),
    //             c2.cards.iter().collect::<String>()
    //         );

    //         println!("Before sort:");
    //         for hand in hands.iter() {
    //             println!("{:?}", hand.hand.cards.iter().collect::<String>());
    //         }
    //         sort_hands(&mut hands);
    //         println!("After sort:");
    //         for hand in hands.iter() {
    //             println!("{:?}", hand.hand.cards.iter().collect::<String>());
    //         }

    //         let total = hands
    //             .iter()
    //             .enumerate()
    //             .map(|(idx, hand)| {
    //                 let total = hand.bid * (idx + 1);
    //                 total
    //             })
    //             .sum::<usize>();

    //         assert_eq!(total, 6440);
    //     }

    //     #[test]
    //     fn test_input() {
    //         if is_demo_mode() {
    //             return;
    //         }
    //         let input = get_input();
    //         let sum = part_1(&input);

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
            let result = part_1(&input);

            assert_eq!(result, 5905);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }
            let input = get_input();
            let sum = part_1(&input);

            println!("Answer pt2: {}", sum);
        }
    }
}
