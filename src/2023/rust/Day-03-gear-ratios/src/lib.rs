use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Number {
    coords: Vec<Coords>,
    number: Option<String>,
    is_valid: bool,
}

impl Number {
    fn empty() -> Self {
        Self {
            coords: Vec::new(),
            number: None,
            is_valid: false,
        }
    }

    fn add_digit(&mut self, digit: char) {
        if self.number.is_none() {
            self.number = Some(String::new());
        }

        self.number.as_mut().unwrap().push(digit);
    }

    fn add_coords(&mut self, coords: Coords) {
        self.coords.push(coords);
    }

    fn set_valid(&mut self) {
        self.is_valid = true;
    }
}

#[derive(Debug)]
struct SchematicMap {
    symbols: HashSet<Coords>,
    numbers: Vec<Number>,
    gears: Vec<Coords>,
}

impl SchematicMap {
    /// Check if they have a symbol in their surroundings
    fn validate_numbers(&mut self) -> u32 {
        let mut sum = 0;
        for number in self.numbers.iter_mut() {
            if number.is_valid {
                sum += number.number.as_ref().unwrap().parse::<u32>().unwrap();
                continue;
            }

            // Check if the number has a symbol in its surroundings
            let mut has_symbol = false;
            for coords in number.coords.iter() {
                for x in (coords.x.saturating_sub(1))..=(coords.x.saturating_add(1)) {
                    for y in (coords.y.saturating_sub(1))..=(coords.y.saturating_add(1)) {
                        if self.symbols.contains(&Coords { x, y }) {
                            has_symbol = true;
                            break;
                        }
                    }

                    if has_symbol {
                        break;
                    }
                }
            }

            if has_symbol {
                number.set_valid();
                sum += number.number.as_ref().unwrap().parse::<u32>().unwrap();
            }
        }
        sum
    }

    fn find_gear_ratio_sum(&self) -> u32 {
        let mut gear_ratio_sum = 0;

        // Check for every gear if it has 2 numbers in its surroundings

        for g_coords in self.gears.iter() {
            let mut coords_to_check: Vec<Coords> = Vec::new();

            // Find all possible empty spaces or numbers in the surroundings
            for x in (g_coords.x.saturating_sub(1))..=(g_coords.x.saturating_add(1)) {
                for y in (g_coords.y.saturating_sub(1))..=(g_coords.y.saturating_add(1)) {
                    // Skip the gear itself
                    if x == g_coords.x && y == g_coords.y {
                        continue;
                    }

                    // Skip any symbol
                    if self.symbols.contains(&Coords { x, y }) {
                        continue;
                    }

                    // Possible empty space or number
                    coords_to_check.push(Coords { x, y });
                }
            }

            // Check with every number if its in its surroudings
            let mut numbers_in_surroundings = 0;
            let mut gear_ratio = 1;
            for number in self.numbers.iter() {
                let mut gear_has_this_number = false;

                for coords in number.coords.iter() {
                    if coords_to_check.contains(coords) {
                        gear_has_this_number = true;
                        break;
                    }
                }

                // Add it to the sum if the gear has this number
                if gear_has_this_number {
                    numbers_in_surroundings += 1;
                    gear_ratio *= number.number.as_ref().unwrap().parse::<u32>().unwrap();
                }

                if numbers_in_surroundings > 2 {
                    break;
                }
            }

            if numbers_in_surroundings == 2 {
                gear_ratio_sum += gear_ratio;
            }
        }

        gear_ratio_sum
    }
}

/// Parses the input into a SchematicMap
/// E.g.:
///
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
fn parse_input(input: &str) -> SchematicMap {
    let mut symbols: HashSet<Coords> = HashSet::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<Coords> = Vec::new();

    let mut number: Option<Number> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            match symbol {
                // Skip empty spaces
                '.' => {
                    if let Some(number) = number.take() {
                        numbers.push(number.clone());
                    }
                }
                // Add digit to number
                '0'..='9' => {
                    if number.is_none() {
                        number = Some(Number::empty());
                    }
                    let number = number.as_mut().unwrap();
                    number.add_digit(symbol);
                    number.add_coords(Coords { x, y });
                }
                // Mark a symbol in the hash set
                symbol => {
                    if let Some(number) = number.take() {
                        numbers.push(number.clone());
                    }
                    if symbol == '*' {
                        gears.push(Coords { x, y });
                    }
                    symbols.insert(Coords { x, y });
                }
            }
        }
    }

    SchematicMap {
        symbols,
        numbers,
        gears,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    pub mod part1 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();
            let mut map = parse_input(&input);

            let sum = map.validate_numbers();

            // println!("Invalid numbers");
            // for number in map.numbers.iter().filter(|n| !n.is_valid) {
            //     println!("Number: {:?}", number.number);
            // }

            assert_eq!(sum, 4361);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let mut map = parse_input(&input);
            let sum = map.validate_numbers();

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
            let mut map = parse_input(&input);
            let gear_ratio_sum = map.find_gear_ratio_sum();

            assert_eq!(gear_ratio_sum, 467835);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let mut map = parse_input(&input);
            let gear_ratio_sum = map.find_gear_ratio_sum();

            println!("Answer pt2: {}", gear_ratio_sum);
        }
    }
}
