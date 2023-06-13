use std::fs;

use rucksack_reorganization::{
    find_common_item_in_elf_group, find_common_item_in_rucksack, init_find_common_elf_group,
    Priority,
};

fn main() {
    let rucksacks = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut sum = 0;
    for rucksack in rucksacks.lines() {
        let common_item = find_common_item_in_rucksack(rucksack);
        sum += common_item.priority();
    }
    println!("Sum priorities: {}", sum);

    let mut sum = 0;

    let rucksacks_lines: Vec<&str> = rucksacks.lines().collect();

    for rucksack_group in rucksacks_lines.chunks(3) {
        let common_item = init_find_common_elf_group!(rucksack_group)
            .expect("Every group must have a common item");

        sum += common_item.priority();
    }
    println!("Sum priorities: {}", sum);
}
