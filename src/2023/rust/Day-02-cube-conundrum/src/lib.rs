#[derive(Debug)]
struct Configuration {
    red: u32,
    green: u32,
    blue: u32,
}

impl Configuration {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
enum Colors {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Colors {
    fn from_text(text: &str, value: u32) -> Colors {
        match text {
            "red" => Colors::Red(value),
            "green" => Colors::Green(value),
            "blue" => Colors::Blue(value),
            _ => panic!("Unknown color"),
        }
    }
}

#[derive(Debug)]
struct Move {
    red: u32,
    green: u32,
    blue: u32,
}

impl Move {
    fn new(red: u32, green: u32, blue: u32) -> Move {
        Move { red, green, blue }
    }

    fn empty() -> Move {
        Move::new(0, 0, 0)
    }

    fn add_color(&mut self, color: Colors) {
        match color {
            Colors::Red(value) => self.red += value,
            Colors::Green(value) => self.green += value,
            Colors::Blue(value) => self.blue += value,
        }
    }
}

#[derive(Debug)]
struct Game {
    moves: Vec<Move>,
    id: u32,
}

fn is_valid_move(movement: &Move, configuration: &Configuration) -> bool {
    /// Returns true if the move is valid, false otherwise.
    !(movement.red > configuration.red
        || movement.green > configuration.green
        || movement.blue > configuration.blue)
}

fn is_valid_game(game: &Game, configuration: &Configuration) -> bool {
    /// Returns true if the game is valid, false otherwise.
    game.moves
        .iter()
        .all(|move_| is_valid_move(move_, configuration))
}

/// Parse move given a game:
/// e.g: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_line(line: &str) -> Game {
    // Split the line into the game id and the moves
    let mut split_game = line.split(":");
    let game_id_string = split_game.next().unwrap();
    let all_moves_str = split_game.next().unwrap().split(";");

    // Get the game id
    let id = game_id_string
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse::<u32>()
        .expect("Could not parse game id");

    // Parse all the moves
    let moves = all_moves_str
        .map(|moves| {
            let results = moves
                .split(",")
                .map(|x| {
                    let splits = x.split_whitespace().collect::<Vec<&str>>();
                    let color_name = splits[1];
                    let value = splits[0].parse::<u32>().expect("Could not parse value");

                    let c = Colors::from_text(color_name, value);
                    c
                })
                .collect::<Vec<Colors>>();

            let mut move_ = Move::empty();
            for color in results {
                move_.add_color(color);
            }
            move_
        })
        .collect::<Vec<Move>>();

    Game { moves, id }
}

fn find_minimum_configuration(game: &Game) -> Configuration {
    let mut minimum_configuration = Configuration {
        red: 0,
        green: 0,
        blue: 0,
    };

    for move_ in &game.moves {
        minimum_configuration.red = minimum_configuration.red.max(move_.red);
        minimum_configuration.green = minimum_configuration.green.max(move_.green);
        minimum_configuration.blue = minimum_configuration.blue.max(move_.blue);
    }

    minimum_configuration
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    const initial_config: Configuration = Configuration {
        red: 12,
        green: 13,
        blue: 14,
    };

    pub mod part1 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();

            let mut sum = 0;
            for line in input.lines() {
                let game = parse_line(line);
                let is_valid_game = is_valid_game(&game, &initial_config);

                if !is_valid_game {
                    continue;
                }
                sum += game.id
            }

            assert_eq!(sum, 8);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();

            let mut sum = 0;
            for line in input.lines() {
                let game = parse_line(line);
                let is_valid_game = is_valid_game(&game, &initial_config);

                if !is_valid_game {
                    continue;
                }
                sum += game.id
            }

            println!("Answer pt1: {}", sum);
        }
    }

    pub mod part2 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();

            let mut sum = 0;
            for line in input.lines() {
                let game = parse_line(line);
                let min_config = find_minimum_configuration(&game);
                println!("Game: {}, min_config: {:?}", game.id, min_config);
                println!("Power: {}", min_config.power());
                sum += min_config.power();
            }

            assert_eq!(sum, 2286);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();

            let mut sum = 0;
            for line in input.lines() {
                let game = parse_line(line);
                let min_config = find_minimum_configuration(&game);
                sum += min_config.power();
            }

            println!("Answer pt2: {}", sum);
        }
    }
}
