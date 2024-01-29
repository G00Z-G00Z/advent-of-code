#![allow(dead_code, non_snake_case)]

use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Coord { x, y } = self;
        write!(f, "({}, {})", x, y)
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Map {
    galaxies: Vec<Coord>,
    width: usize,
    height: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Make string with just dots

        let row = ".".repeat(self.width);
        let mut map = vec![row; self.height];

        // Add galaxies

        for galaxy in &self.galaxies {
            let Coord { x: col, y: row } = galaxy;
            let row = *row as usize;
            let col = *col as usize;
            map.get_mut(row).unwrap().replace_range(col..=col, "#");
        }

        let mut map = map.join("\n");

        map.push('\n');

        write!(f, "{}", map)
    }
}

impl Map {
    fn new(galaxies: Vec<Coord>, width: usize, height: usize) -> Self {
        Self {
            galaxies,
            width,
            height,
        }
    }

    fn print_numbered(&self) {
        // Make string with just dots

        let row = ".".repeat(self.width);
        let mut map = vec![row; self.height];

        // Add galaxies

        for (idx, galaxy) in self.galaxies.iter().enumerate() {
            let Coord { x: col, y: row } = galaxy;
            let row = *row as usize;
            let col = *col as usize;
            map.get_mut(row)
                .unwrap()
                .replace_range(col..=col, &(idx + 1).to_string());
        }

        let mut map = map.join("\n");

        map.push('\n');

        println!("{}", map);
    }

    fn find_sum_min_distance(&self) -> usize {
        let combinations = self.galaxies.iter().combinations(2);

        combinations
            .map(|galaxies| distance_between_galaxies(&galaxies[0], &galaxies[1]))
            .sum()
    }
}

fn distance_between_galaxies(n1: &Coord, n2: &Coord) -> usize {
    ((n2.x - n1.x).abs() + (n2.y - n1.y)) as usize
}

fn find_galaxies(input: &str) -> Map {
    let mut galaxies = Vec::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Coord::new(x as i32, y as i32));
            }
        }
    }

    Map::new(galaxies, width, height)
}

fn find_rows_cols_without_galaxies(map: &Map) -> (Vec<i32>, Vec<i32>) {
    // Find rows and cols without galaxies
    // Returns:
    // 1. A vector of rows without galaxies
    // 2. A vector of cols without galaxies
    let mut rows_with_galaxies = HashSet::new();
    let mut cols_with_galaxies = HashSet::new();

    for galaxy in &map.galaxies {
        rows_with_galaxies.insert(galaxy.y);
        cols_with_galaxies.insert(galaxy.x);
    }

    let rows_without_galaxies: Vec<i32> = (0..map.height as i32)
        .filter(|row| !rows_with_galaxies.contains(row))
        .collect();

    let cols_without_galaxies: Vec<i32> = (0..map.width as i32)
        .filter(|col| !cols_with_galaxies.contains(col))
        .collect();

    (rows_without_galaxies, cols_without_galaxies)
}

fn update_galaxy_with_light_year(
    map: Map,
    empty_rows: Vec<i32>,
    empty_col: Vec<i32>,
    factor_of_expansion: i32,
) -> Map {
    // Makes the empty rows and cols twice as big (add 1 more space for each empty row/col)
    let mut map = map;

    let mut empty_rows = empty_rows;
    let mut empty_col = empty_col;
    let factor_of_expansion = factor_of_expansion - 1;

    // Sort them to make sure we add the rows/cols in the correct order
    empty_rows.sort();
    empty_col.sort();

    // Add the index to each row to compensate for the rows that are added before
    for i in 0..empty_rows.iter().len() {
        let row = (empty_rows[i] + (i as i32 * factor_of_expansion)) as i32;

        for galaxy in map.galaxies.iter_mut() {
            if galaxy.y > row {
                galaxy.y += factor_of_expansion;
            }
        }
    }

    for i in 0..empty_col.iter().len() {
        let col = (empty_col[i] + (i as i32 * factor_of_expansion)) as i32;

        for galaxy in map.galaxies.iter_mut() {
            if galaxy.x > col {
                galaxy.x += factor_of_expansion;
            }
        }
    }

    Map::new(
        map.galaxies,
        map.width + empty_col.len(),
        map.height + empty_rows.len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    // pub mod part1 {

    //     use std::fs;

    //     use super::*;

    //     // #[test]
    //     // fn test_demo_input() {
    //     //     if !is_demo_mode() {
    //     //         return;
    //     //     }

    //     //     let input = get_input();
    //     //     let map = find_galaxies(&input);

    //     //     let map_str = format!("{}", map);
    //     //     assert_eq!(map_str, input, "Map is not returning input");

    //     //     let (empty_rows, empty_col) = find_rows_cols_without_galaxies(&map);
    //     //     let map = update_galaxy_with_light_year(map, empty_rows, empty_col, 2);
    //     //     let map_str = format!("{}", map);

    //     //     let input_expanded = fs::read_to_string("demo-expanded-p1.txt")
    //     //         .expect("Failed to find expanded demo file");

    //     //     assert_eq!(
    //     //         map_str, input_expanded,
    //     //         "Expanded map is not properly expanded"
    //     //     );

    //     //     println!("Expanded map:\n{}", map_str);
    //     //     println!("Numbered");
    //     //     map.print_numbered();

    //     //     // assert_eq!(input, "hey");
    //     //     let answers = [((5, 9), 9), ((1, 7), 15), ((8, 9), 5)];

    //     //     let answers = answers
    //     //         .iter()
    //     //         .map(|((n1, n2), answer)| {
    //     //             let n1 = map.galaxies.get(*n1 - 1).unwrap();
    //     //             let n2 = map.galaxies.get(*n2 - 1).unwrap();

    //     //             ((n1, n2), answer)
    //     //         })
    //     //         .collect::<Vec<_>>();

    //     //     for ((n1, n2), answer) in answers {
    //     //         let distance = distance_between_galaxies(n1, n2);
    //     //         // println!("Distance between {:?} and {:?}: {}", n1, n2, distance);
    //     //         assert_eq!(distance, *answer, "Distance should be: {}", *answer);
    //     //     }

    //     //     let min_distance = map.find_sum_min_distance();
    //     //     assert_eq!(min_distance, 374);
    //     // }

    //     //     #[test]
    //     //     fn test_input() {
    //     //         if is_demo_mode() {
    //     //             return;
    //     //         }

    //     //         let input = get_input();
    //     //         let map = find_galaxies(&input);
    //     //         let (empty_rows, empty_col) = find_rows_cols_without_galaxies(&map);
    //     //         let map = update_galaxy_with_light_year(map, empty_rows, empty_col);
    //     //         let min_distance = map.find_sum_min_distance();
    //     //         println!("Answer pt1: {}", min_distance);
    //     //     }
    // }

    pub mod part2 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let answers = [(2, 374), (10, 1030), (100, 8410)];

            let input = get_input();
            let map = find_galaxies(&input);
            let (empty_rows, empty_col) = find_rows_cols_without_galaxies(&map);

            for (factor, result) in answers.iter() {
                println!("Factor: {}", factor);
                let map = update_galaxy_with_light_year(
                    map.clone(),
                    empty_rows.clone(),
                    empty_col.clone(),
                    *factor,
                );
                let min_distance = map.find_sum_min_distance();
                assert_eq!(
                    min_distance, *result,
                    "For factor of {}, distance should be {}",
                    factor, result
                );
            }
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let map = find_galaxies(&input);
            let (empty_rows, empty_col) = find_rows_cols_without_galaxies(&map);
            let map = update_galaxy_with_light_year(map, empty_rows, empty_col, 1000000);
            let min_distance = map.find_sum_min_distance();
            println!("Answer pt2: {}", min_distance);
        }
    }
}
