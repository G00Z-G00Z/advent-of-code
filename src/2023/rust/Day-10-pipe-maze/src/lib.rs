#![allow(dead_code, non_snake_case)]

use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum MapSymbols {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Starting,
}

impl Display for MapSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            MapSymbols::VerticalPipe => '|',
            MapSymbols::HorizontalPipe => '-',
            MapSymbols::NorthEastBend => 'L',
            MapSymbols::NorthWestBend => 'J',
            MapSymbols::SouthWestBend => '7',
            MapSymbols::SouthEastBend => 'F',
            MapSymbols::Ground => '.',
            MapSymbols::Starting => 'S',
        };
        write!(f, "{}", symbol)
    }
}

impl MapSymbols {
    fn from_char(char: char) -> MapSymbols {
        match char {
            '|' => MapSymbols::VerticalPipe,
            '-' => MapSymbols::HorizontalPipe,
            'L' => MapSymbols::NorthEastBend,
            'J' => MapSymbols::NorthWestBend,
            '7' => MapSymbols::SouthWestBend,
            'F' => MapSymbols::SouthEastBend,
            '.' => MapSymbols::Ground,
            'S' => MapSymbols::Starting,
            _ => panic!("Unknown map symbol: {}", char),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }
}

#[derive(Debug, Clone)]
struct Node {
    coords: Coords,
    symbol: MapSymbols,
    conections: RefCell<Vec<Coords>>,
}

impl Node {
    fn new(x: i32, y: i32, symbol: MapSymbols) -> Node {
        Node {
            coords: Coords::new(x, y),
            symbol,
            conections: RefCell::new(Vec::new()),
        }
    }

    fn open_coords(&self) -> Vec<Option<Coords>> {
        // [N, W, S, E]
        let mask = match self.symbol {
            MapSymbols::VerticalPipe => [true, false, true, false],
            MapSymbols::HorizontalPipe => [false, true, false, true],
            MapSymbols::NorthEastBend => [true, false, false, true],
            MapSymbols::NorthWestBend => [true, true, false, false],
            MapSymbols::SouthWestBend => [false, true, true, false],
            MapSymbols::SouthEastBend => [false, false, true, true],
            MapSymbols::Starting | MapSymbols::Ground => return vec![],
        };

        let mut coords: Vec<Option<Coords>> = Vec::with_capacity(2);

        if mask[0] {
            coords.push(Some(Coords::new(self.coords.x, self.coords.y - 1)));
        }

        if mask[1] {
            coords.push(Some(Coords::new(self.coords.x - 1, self.coords.y)));
        }

        if mask[2] {
            coords.push(Some(Coords::new(self.coords.x, self.coords.y + 1)));
        }

        if mask[3] {
            coords.push(Some(Coords::new(self.coords.x + 1, self.coords.y)));
        }
        coords
    }
}

struct Map(Vec<Vec<Node>>);

impl Map {
    fn get(&self, coords: &Coords) -> Option<&Node> {
        self.0
            .get(coords.y as usize)
            .and_then(|row| row.get(coords.x as usize))
    }

    fn new(map: Vec<Vec<Node>>) -> Map {
        Map(map)
    }

    fn print_list_of_nodes_and_animal(&self, animal: &Vec<Coords>, nodes: &Vec<Coords>) {
        for row in self.0.iter() {
            for node in row.iter() {
                if nodes.contains(&node.coords) {
                    print!("*");
                } else if animal.contains(&node.coords) {
                    print!("{}", node.symbol);
                } else {
                    print!("x");
                }
            }
            println!();
        }
    }

    fn change_staring_to_symbol<'a>(&'a self, animal: &'a Vec<Coords>) -> MapSymbols {
        let starting_node = animal
            .iter()
            .find(|coords| {
                let node = self.get(coords).unwrap();
                if let MapSymbols::Starting = node.symbol {
                    true
                } else {
                    false
                }
            })
            .expect("Not founding node");

        let connections = self.get(starting_node).unwrap().conections.borrow();

        let (coord1, coord2) = (&connections[0], &connections[1]);

        match (coord1.x, coord1.y, coord2.x, coord2.y) {
            (x1, _, x2, _) if x1 == x2 => MapSymbols::HorizontalPipe,
            (_, y1, _, y2) if y1 == y2 => MapSymbols::VerticalPipe,
            (x1, y1, x2, y2) if x1 < x2 => {
                //
                //     ~   | * | ~
                //  x1, y1 | S | ~
                //     ~   | * | ~
                //

                if y1 < y2 {
                    MapSymbols::NorthEastBend
                } else {
                    MapSymbols::SouthWestBend
                }
            }
            (x1, y1, x2, y2) if x1 > x2 => {
                //
                //     ~   | * | ~
                //     ~   | S |x1, y1
                //     ~   | * | ~
                //

                if y1 < y2 {
                    MapSymbols::SouthEastBend
                } else {
                    MapSymbols::NorthWestBend
                }
            }
            (_, _, _, _) => unreachable!("Unknown starting node"),
        }
    }
}

fn parse_input(input: &str) -> (Map, Node) {
    let mut map = Vec::new();
    let mut starting_node: Option<Node> = None;

    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Node> = Vec::new();

        for (x, char) in line.chars().enumerate() {
            let symbol = MapSymbols::from_char(char);
            let node = Node::new(x as i32, y as i32, symbol);
            row.push(node.clone());

            match node.symbol {
                MapSymbols::Starting => {
                    starting_node = Some(node);
                }
                _ => {}
            }
        }

        map.push(row);
    }

    (
        Map::new(map),
        starting_node.expect("No starting node found"),
    )
}

fn connect_nodes(map: &Map) {
    let mut starting_node = None;
    map.0.iter().flatten().for_each(|node| {
        match node.symbol {
            MapSymbols::Starting => {
                starting_node = Some(node);
            }
            _ => {}
        }

        let possible_connections = node.open_coords();

        for coord in possible_connections {
            if let Some(coord) = coord {
                let other_node = map.get(&coord);

                if let Some(other_node) = other_node {
                    let other_nodes_possible_conections = other_node.open_coords();

                    if other_nodes_possible_conections.contains(&Some(node.coords.clone())) {
                        node.conections.borrow_mut().push(other_node.coords.clone());
                    }
                }
            }
        }
    });

    // Handle starting node

    let starting_node = starting_node.expect("No starting node found");

    let (x, y) = (starting_node.coords.x, starting_node.coords.y);

    for j in y - 1..=y + 1 {
        for i in x - 1..=x + 1 {
            if i == x && j == y {
                continue;
            }

            let other_node = map.get(&Coords::new(i, j));

            if other_node.is_none() {
                continue;
            }

            let other_node = other_node.unwrap();
            let possible_connections = other_node.open_coords();

            for coord in possible_connections.iter().filter_map(|c| {
                if let Some(c) = c {
                    return Some(c);
                }

                None
            }) {
                if !(coord.x == x && coord.y == y) {
                    continue;
                }

                // Add connection to both nodes
                starting_node
                    .conections
                    .borrow_mut()
                    .push(other_node.coords.clone());
                other_node
                    .conections
                    .borrow_mut()
                    .push(starting_node.coords.clone());
            }
        }
    }
}

fn find_animal_with_distance<'a>(map: &'a Map, starting_node: &'a Node) -> Vec<&'a Node> {
    // Finds the node in the animal and their distance to the starting node
    let mut visited = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(starting_node.coords.clone());

    while let Some(coords) = queue.pop_front() {
        let node = map.get(&coords);

        if node.is_none() {
            continue;
        }

        let node = node.unwrap();

        if visited.contains(&node.coords) {
            continue;
        }

        visited.push(node.coords.clone());

        for connection in node.conections.borrow().iter() {
            queue.push_back(connection.clone());
        }
    }

    visited
        .iter()
        .map(|coord| map.get(coord).expect("Visited unkown node"))
        .collect()
}

fn find_ground_group_area<'a>(
    only_grounds: &'a HashSet<Coords>,
    starting_node: &'a Coords,
    group: &mut HashSet<Coords>,
) {
    // Finds all the neighboring ground nodes starting from the starting node

    // Check if its on the group
    if group.contains(starting_node) {
        return;
    }

    // Not grounds are not allowed
    if !only_grounds.contains(starting_node) {
        return;
    }

    // Add it
    group.insert(starting_node.clone());

    let Coords { x, y } = starting_node;

    for i in (x - 1..x + 1).step_by(2) {
        find_ground_group_area(only_grounds, &Coords::new(i, *y), group);
    }
    for j in (y - 1..y + 1).step_by(2) {
        find_ground_group_area(only_grounds, &Coords::new(*x, j), group);
    }
}

fn find_enclosed_space<'a>(
    map: &'a Map,
    animal: &Vec<&'a Node>,
    starting_node: &'a Node,
) -> Vec<Coords> {
    // TODO: Any tile can be enclosed!
    // Finds ground enclosed by animal
    // It uses ray casting algorithm to do it

    let animal_set = animal
        .iter()
        .map(|node| node.coords.clone())
        .collect::<HashSet<_>>();
    let mut enclosed = Vec::new();

    let all_tiles = map
        .0
        .iter()
        .flatten()
        .filter(|node| !animal_set.contains(&node.coords));

    for tile in all_tiles {
        let coords = tile.coords.clone();

        // Go up and count the times it touches the animal
        let mut east_touches = 0;
        let mut west_touches = 0;

        let Coords { x, mut y } = coords.clone();

        let is_staring_in_the_way =
            coords.x == starting_node.coords.x && coords.y < starting_node.coords.y;

        let condition = |y: i32| -> bool {
            if is_staring_in_the_way {
                return y < map.0.len() as i32;
            }
            return y >= 0;
        };

        let change_y = |y: &mut i32| {
            if is_staring_in_the_way {
                *y += 1;
            } else {
                *y -= 1;
            }
        };

        while condition(y) {
            let node = animal_set.get(&Coords::new(x, y));

            if let Some(node) = node {
                let node = map.get(node).unwrap();

                let mut found = false;
                let symbol = {
                    if let MapSymbols::Starting = node.symbol {
                        let coordsAnimal =
                            animal.iter().map(|n| n.coords.clone()).collect::<Vec<_>>();
                        found = true;
                        MapSymbols::NorthEastBend
                    } else {
                        node.symbol.clone()
                    }
                };

                if found {
                    println!("Found staring in the way: {}", symbol);
                }

                match symbol {
                    MapSymbols::HorizontalPipe => {
                        west_touches += 1;
                        east_touches += 1;
                    }
                    MapSymbols::NorthEastBend | MapSymbols::SouthEastBend => {
                        east_touches += 1;
                    }

                    MapSymbols::NorthWestBend | MapSymbols::SouthWestBend => {
                        west_touches += 1;
                    }
                    _ => {}
                }
            }
            change_y(&mut y);
        }

        if [east_touches, west_touches].iter().any(|t| *t % 2 != 0) {
            enclosed.push(coords.clone());
        }
    }

    enclosed
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
            let (map, starting) = parse_input(&input);
            connect_nodes(&map);
            let animal = find_animal_with_distance(&map, &starting);
            let max_distance = animal.len() / 2;

            assert_eq!(max_distance, 8);
        }

        //     #[test]
        //     fn test_input() {
        //         if is_demo_mode() {
        //             return;
        //         }

        //         let input = get_input();
        //         let (map, starting) = parse_input(&input);
        //         connect_nodes(&map);
        //         let animal = find_animal_with_distance(&map, &starting);
        //         let max_distance = animal.len() / 2;

        //         println!("Answer pt1: {}", max_distance);
        //     }
    }

    pub mod part2 {

        use std::fs;

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let answers = [
                // ("demo-input-part-2-1.txt", 4),
                // ("demo-input-part-2-2.txt", 4),
                // ("demo-input-part-2-3.txt", 8),
                ("demo-input-part-2-4.txt", 10),
            ];

            for (idx, (input, answer)) in answers.iter().enumerate() {
                let input = fs::read_to_string(input)
                    .expect(format!("{} failed to read file", input).as_str());
                println!("Test: {}", idx + 1);
                println!("{}", input);

                let (map, starting) = parse_input(&input);
                connect_nodes(&map);
                let animal = find_animal_with_distance(&map, &starting);
                let enclosed_space = find_enclosed_space(&map, &animal, &starting);

                let animal_coords = animal
                    .iter()
                    .map(|node| node.coords.clone())
                    .collect::<Vec<_>>();

                map.print_list_of_nodes_and_animal(&animal_coords, &enclosed_space);

                // for node in enclosed_space.iter() {
                //     println!("{:?}", node);
                // }

                assert_eq!(enclosed_space.len(), *answer, "Test {} failed", idx + 1);
            }
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let (map, starting) = parse_input(&input);
            connect_nodes(&map);
            let animal = find_animal_with_distance(&map, &starting);
            let enclosed_space = find_enclosed_space(&map, &animal, &starting);

            let animal_coords = animal
                .iter()
                .map(|node| node.coords.clone())
                .collect::<Vec<_>>();

            map.print_list_of_nodes_and_animal(&animal_coords, &enclosed_space);

            println!("Answer pt2: {}", enclosed_space.len());
        }
    }
}
