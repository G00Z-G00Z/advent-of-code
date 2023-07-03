#[derive(PartialEq, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

pub enum GnomeStrategy {
    NeedToWin,
    NeedToLose,
    NeedToDraw,
}

#[derive(PartialEq, Debug)]
pub enum Outcome {
    Win(Move),
    Lose(Move),
    Draw(Move),
}

impl From<char> for GnomeStrategy {
    fn from(s: char) -> Self {
        match s.to_ascii_lowercase() {
            'x' => GnomeStrategy::NeedToLose,
            'y' => GnomeStrategy::NeedToDraw,
            'z' => GnomeStrategy::NeedToWin,
            x => panic!("{} is not a valid letter", x),
        }
    }
}

impl GnomeStrategy {
    pub fn move_for(&self, opponent: &Move) -> Move {
        match self {
            GnomeStrategy::NeedToWin => match opponent {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            GnomeStrategy::NeedToDraw => match opponent {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors,
            },
            GnomeStrategy::NeedToLose => match opponent {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
        }
    }
}

impl Move {
    fn value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Outcome {
    pub fn value(&self) -> u32 {
        match self {
            Outcome::Win(m) => 6 + m.value(),
            Outcome::Draw(m) => 3 + m.value(),
            Outcome::Lose(m) => 0 + m.value(),
        }
    }
}

impl Move {
    pub fn beats(&self, opponent: &Move) -> Outcome {
        match self {
            Move::Rock => match opponent {
                Move::Rock => Outcome::Draw(Move::Rock),
                Move::Paper => Outcome::Lose(Move::Rock),
                Move::Scissors => Outcome::Win(Move::Rock),
            },
            Move::Paper => match opponent {
                Move::Rock => Outcome::Win(Move::Paper),
                Move::Paper => Outcome::Draw(Move::Paper),
                Move::Scissors => Outcome::Lose(Move::Paper),
            },
            Move::Scissors => match opponent {
                Move::Rock => Outcome::Lose(Move::Scissors),
                Move::Paper => Outcome::Win(Move::Scissors),
                Move::Scissors => Outcome::Draw(Move::Scissors),
            },
        }
    }
}

impl From<char> for Move {
    fn from(s: char) -> Self {
        match s.to_ascii_lowercase() {
            'a' | 'x' => Move::Rock,
            'b' | 'y' => Move::Paper,
            'c' | 'z' => Move::Scissors,
            x => panic!("{} is not a valid move", x),
        }
    }
}

// make test module

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let moves = vec![
            (Move::from('A'), Move::from('Y')),
            (Move::from('b'), Move::from('x')),
            (Move::from('c'), Move::from('z')),
        ];

        println!("{:?}", moves);

        let scores = moves
            .iter()
            .map(|(opponent, you)| you.beats(opponent))
            .collect::<Vec<Outcome>>();

        println!("{:?}", scores);

        // Print all the scores
        for score in scores.iter() {
            println!("{}", score.value());
        }

        let score = scores.iter().fold(0, |acc, x| acc + x.value());

        assert_eq!(score, 15);
    }
}
