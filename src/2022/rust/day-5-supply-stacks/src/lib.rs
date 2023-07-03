pub type Crate = Vec<char>;

pub type Crates = Vec<Crate>;

#[derive(Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub how_many: usize,
}

pub fn move_crate(crates: &mut Crates, instruction: Move) {
    let from = crates
        .get_mut(instruction.from - 1)
        .expect("No crate at from");
    let slice = from.split_off(from.len() - instruction.how_many);

    let to = crates.get_mut(instruction.to - 1).expect("No crate at to");
    to.extend(slice);
}

/// Takes in the input of a crates string and returns a vector of crates
pub fn parse_crates(input: &str) -> Crates {
    let lines = input.lines().collect::<Vec<&str>>();

    let id_line = lines.last().expect("No id lines found");

    let total_crates = id_line.split_whitespace().collect::<String>().len();

    let mut crates: Crates = vec![];

    // Initialize the crates
    for _ in 0..total_crates {
        crates.push(vec![]);
    }

    for possible_cells in lines.iter().rev().skip(1).filter(|line| !line.is_empty()) {
        let cell_values: Crate = possible_cells.chars().skip(1).step_by(4).collect();

        for i in 0..total_crates {
            let cell = cell_values.get(i).expect("No cell found");
            if *cell == ' ' {
                continue;
            }
            let crate_to_add = crates.get_mut(i).expect("No crate found");
            crate_to_add.push(*cell);
        }
    }

    crates
}

pub fn parse_movements(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    for l in input.lines().filter(|l| !l.is_empty()) {
        let fields = l.split_whitespace().collect::<Vec<&str>>();
        let (how_many, from, to) = (fields[1], fields[3], fields[5]);
        let (how_many, from, to) = (
            how_many.parse().unwrap(),
            from.parse().unwrap(),
            to.parse().unwrap(),
        );
        moves.push(Move { from, to, how_many })
    }

    moves
}

// Tests
#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_example() {
        let input =
            fs::read_to_string("input_mock.txt").expect("Something went wrong reading the file");

        let mut groups = input.split("\n\n");

        let crates = groups.next().expect("No crates found");
        let moves = groups.next().expect("No moves found");

        let mut crates = parse_crates(crates);
        let moves = parse_movements(moves);

        for m in moves {
            println!("Before\n {:?}", crates);
            println!("Move {:?}", m);
            move_crate(&mut crates, m);
            println!("After\n {:?}", crates);
        }

        let final_input = crates
            .iter()
            .map(|c| c.last().unwrap_or(&' '))
            .collect::<String>();

        assert_eq!(final_input, "CMZ");
    }

    #[test]
    #[ignore]
    fn test_parsing_movement() {
        let crates_raw = "
move 2 from 2 to 7
move 8 from 5 to 6
move 2 from 4 to 5
move 1 from 4 to 5
";
        let movements = parse_movements(&crates_raw);
        println!("{:?}", movements);
    }

    #[test]
    #[ignore]
    fn test_parsing_crates() {
        let crates_raw = "
                [V]     [C]     [M]
[V]     [J]     [N]     [H]     [V]
[R] [F] [N]     [W]     [Z]     [N]
[H] [R] [D]     [Q] [M] [L]     [B]
[B] [C] [H] [V] [R] [C] [G]     [R]
[G] [G] [F] [S] [D] [H] [B] [R] [S]
[D] [N] [S] [D] [H] [G] [J] [J] [G]
[W] [J] [L] [J] [S] [P] [F] [S] [L]
 1   2   3   4   5   6   7   8   9 
";

        let crates = parse_crates(&crates_raw);
        println!("{:?}", crates);
    }
}
