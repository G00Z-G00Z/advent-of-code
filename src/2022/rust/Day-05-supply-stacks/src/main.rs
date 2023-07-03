use std::fs;

use supply_stacks::{move_crate, parse_crates, parse_movements};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut groups = input.split("\n\n");

    let crates = groups.next().expect("No crates found");
    let moves = groups.next().expect("No moves found");

    let mut crates = parse_crates(crates);
    let moves = parse_movements(moves);

    for m in moves {
        move_crate(&mut crates, m);
    }

    for c in crates {
        print!("{}", c.last().unwrap_or(&' '));
    }
}
