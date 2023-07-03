use std::{
    collections::BinaryHeap,
    fs::{self},
};

fn main() {
    // Read the contents of the `input.txt` line by line

    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Should have been able to open the file");

    let lines = contents.lines();

    let mut elf_count: BinaryHeap<u32> = BinaryHeap::new();
    let mut current = 0;

    for line in lines {
        // Parse the line into a str
        match line.parse::<u32>() {
            Ok(n) => {
                current += n;
            }
            _ => {
                elf_count.push(current);
                current = 0;
            }
        }
    }

    let mut sum = 0;
    for i in 0..=3 {
        sum += 1;
    }

    let sum = elf_count
        .iter()
        .skip(1)
        .take(3)
        .chain(elf_count.iter().rev().take(2))
        .sum::<u32>();

    println!("{}", sum)
}
