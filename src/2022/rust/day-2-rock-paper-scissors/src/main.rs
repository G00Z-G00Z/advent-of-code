use std::fs;

use rock_paper_scissors::{GnomeStrategy, Move};

const FILENAME: &str = "input.txt";

fn main() {
    let lines = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");

    let mut score = 0;

    for l in lines.lines() {
        let mut chars = l.chars();

        let opponent = Move::from(chars.next().unwrap());
        let gnome_instruction = GnomeStrategy::from(chars.skip(1).next().unwrap());
        let you = gnome_instruction.move_for(&opponent);

        let outcome = you.beats(&opponent);

        score += outcome.value();
    }

    println!("Score: {}", score);
}
