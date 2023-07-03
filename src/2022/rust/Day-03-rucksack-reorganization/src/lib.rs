use std::collections::HashSet as Set;

pub trait Priority {
    fn priority(&self) -> u32;
}

impl Priority for char {
    fn priority(&self) -> u32 {
        let value = *self as u32;

        if *self >= 'a' && *self <= 'z' {
            return value - 'a' as u32 + 1;
        } else if *self >= 'A' && *self <= 'Z' {
            return value - 'A' as u32 + 27;
        }
        panic!("Invalid character: {}", self);
    }
}

/// Divides the rucksack into two parts, left and right
/// Finds the common letter in both parts
pub fn find_common_item_in_rucksack(rucksack: &str) -> char {
    if rucksack.len() % 2 != 0 {
        panic!("Rucksack length must be even")
    }

    // chunk
    let (left, right) = rucksack.split_at(rucksack.len() / 2);

    let mut checked_letters: Set<char> = Set::new();

    for left in left.chars() {
        if checked_letters.contains(&left) {
            continue;
        }

        for right in right.chars() {
            if left == right {
                return left;
            }
        }

        checked_letters.insert(left);
    }

    unreachable!("No common letter found in rucksack")
}

#[macro_export]
macro_rules! init_find_common_elf_group {
    ($groups:expr) => {
        find_common_item_in_elf_group($groups, None)
    };
}

pub fn find_common_item_in_elf_group(
    rucksacks: &[&str],
    common_item: Option<char>,
) -> Option<char> {
    match rucksacks.len() {
        0 => common_item,
        1 => {
            let rucksack = rucksacks[0];

            if let None = common_item {
                return None;
            }

            let common_item = common_item.unwrap();

            if rucksack.contains(common_item) {
                Some(common_item)
            } else {
                None
            }
        }
        _ => {
            let (current, rest) = rucksacks.split_at(1);

            let current = current[0];

            let mut check_out_items: Set<char> = Set::new();

            match common_item {
                Some(common_item) => {
                    if !current.contains(common_item) {
                        None
                    } else {
                        return find_common_item_in_elf_group(rest, Some(common_item));
                    }
                }
                None => {
                    for item in current.chars() {
                        if check_out_items.contains(&item) {
                            continue;
                        }

                        let common_item = find_common_item_in_elf_group(rest, Some(item));
                        match common_item {
                            Some(common_item) => return Some(common_item),
                            None => {
                                check_out_items.insert(item);
                                continue;
                            }
                        }
                    }
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn test_example_1() {
        let rucksacks = vec![
            ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
            ("PmmdzqPrVvPwwTWBwg", 'P'),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v'),
            ("ttgJtRGJQctTZtZT", 't'),
            ("CrZsJsPPZsGzwwsLwLmpwMDw", 's'),
        ];

        let mut sum = 0_u32;

        for (rucksack, expected) in rucksacks.iter() {
            let actual = find_common_item_in_rucksack(rucksack);
            assert_eq!(*expected, actual);
            let priority = actual.priority();
            println!("{}: {}", actual, priority);
            sum += actual.priority();
        }

        assert_eq!(sum, 157);
    }

    #[test]
    fn test_example_2() {
        let groups = vec![
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            ],
            vec![
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ],
        ];

        let expected = vec!['r', 'Z'];

        for (group, expect) in groups.iter().zip(expected.iter()) {
            let common = find_common_item_in_elf_group(&group, None);
            println!("group: {:?}", group);
            println!("common: {:?}", common);
            assert_eq!(common, Some(*expect))
        }
    }
}
