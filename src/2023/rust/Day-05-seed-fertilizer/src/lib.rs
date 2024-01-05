use std::ops::Range;

#[derive(Debug)]
struct RangedMap {
    input: Range<u64>,
    output: u64,
}

impl RangedMap {
    fn new(input: u64, output: u64, range: u64) -> Self {
        Self {
            input: input..(input + range),
            output,
        }
    }

    fn map(&self, value: u64) -> u64 {
        if !self.contains(value) {
            return value;
        }

        let value_idx = value - self.input.start;
        let new_value = self.output + value_idx;
        new_value
    }

    fn contains(&self, value: u64) -> bool {
        self.input.contains(&value)
    }
}

#[derive(Debug)]
struct XToYMap {
    maps: Vec<RangedMap>,
}

impl XToYMap {
    fn new(maps: Vec<RangedMap>) -> Self {
        Self { maps }
    }

    fn map(&self, value: u64) -> u64 {
        // Selects the first range that contains the value

        let selected_map = self
            .maps
            .iter()
            .filter(|map| map.contains(value))
            .take(1)
            .next();

        // If a range was found, map the value to the second range
        if let Some(map) = selected_map {
            map.map(value)
        } else {
            value
        }
    }
}

type SeedList = Vec<u64>;

fn map_in_chain(seeds: &SeedList, maps: &Vec<XToYMap>) -> SeedList {
    let mut seeds = seeds.clone();
    for map in maps {
        seeds = seeds.iter().map(|seed| map.map(*seed)).collect();
    }

    seeds
}

fn parse_seed_part_1(input: &str) -> SeedList {
    // E.g. : "seeds: 79 14 55 13"
    input
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<SeedList>()
}

fn parse_seed_part_2(input: &str) -> SeedList {
    // E.g. : "seeds: 79 14 55 13"
    // Where  "seeds: init1 range1 init2 range2 ..."
    // The range is inclusive (E.g. 1 3 = 1, 2, 3)
    input
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .flat_map(|pair| pair[0]..(pair[1] + pair[0]))
        .collect::<SeedList>()
}

fn parse_input_2(input: &str) -> u64 {
    let mut lines = input.split("\n\n");

    let seeds = lines.next().unwrap();

    let mut maps = Vec::new();

    // Make the maps
    for map in lines {
        let mut lines = map.split("\n");

        // First line is the map header
        let _ = lines.next();

        // All other lines are the map ranges
        let mut map_ranges = Vec::new();
        for range in lines {
            let mut range = range.split(" ");
            if range.clone().count() != 3 {
                continue;
            }
            let output = range.next().expect("output").parse::<u64>().unwrap();
            let input = range.next().expect("input").parse::<u64>().unwrap();
            let range = range.next().expect("range").parse::<u64>().unwrap();

            map_ranges.push(RangedMap::new(input, output, range));
        }

        maps.push(XToYMap::new(map_ranges));
    }

    seeds
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|pair| (pair[0], (pair[1] + pair[0])))
        .map(|(begin, end)| end)
        .min()
        .expect("No min value found")
}

fn parse_input<T>(input: &str, seed_parser_fn: T) -> (SeedList, Vec<XToYMap>)
where
    T: Fn(&str) -> SeedList,
{
    let mut maps = Vec::new();

    let mut lines = input.split("\n\n");

    // First line is the seed list:
    // E.g. : "seeds: 79 14 55 13"
    let seeds = seed_parser_fn(lines.next().unwrap());
    println!("Parsed Seeds!!");

    // All other lines are maps of the form:
    // x-to-y map:
    // output_1 input_1 range_1
    // output_2 input_2 range_2
    // ...
    for map in lines {
        let mut lines = map.split("\n");

        // First line is the map header
        let _ = lines.next();

        // All other lines are the map ranges
        let mut map_ranges = Vec::new();
        for range in lines {
            let mut range = range.split(" ");
            if range.clone().count() != 3 {
                continue;
            }
            let output = range.next().expect("output").parse::<u64>().unwrap();
            let input = range.next().expect("input").parse::<u64>().unwrap();
            let range = range.next().expect("range").parse::<u64>().unwrap();

            map_ranges.push(RangedMap::new(input, output, range));
        }

        maps.push(XToYMap::new(map_ranges));
    }

    (seeds, maps)
}

fn get_min_location(seeds: &SeedList, maps: &Vec<XToYMap>) -> u64 {
    let locations = map_in_chain(seeds, maps);
    let min_location = locations.iter().min().unwrap().clone();
    min_location
}

const correct_locations_part_1: [u64; 4] = [82, 43, 86, 35];
const correct_min_location_part_1: u64 = 13;

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    // pub mod part1 {

    //     use super::*;

    //     #[test]
    //     fn test_ranges() {
    //         let x_to_y_map =
    //             XToYMap::new(vec![RangedMap::new(50, 98, 2), RangedMap::new(52, 50, 48)]);

    //         let value = 81;

    //         let new_value = x_to_y_map.map(value);

    //         assert_eq!(new_value, 79);
    //     }

    //     #[test]
    //     fn test_demo_input() {
    //         if !is_demo_mode() {
    //             return;
    //         }

    //         let input = get_input();
    //         let (seeds, maps) = parse_input(&input, parse_seed_part_1);
    //         let locations = map_in_chain(&seeds, &maps);
    //         assert_eq!(&locations[..], correct_locations_part_1);
    //         let min_location = locations.iter().min().unwrap().clone();
    //         assert_eq!(min_location, 35);
    //     }

    //     #[test]
    //     fn test_input() {
    //         if is_demo_mode() {
    //             return;
    //         }

    //         let input = get_input();
    //         let (seeds, maps) = parse_input(&input, parse_seed_part_1);
    //         let min_location = get_min_location(&seeds, &maps);

    //         println!("Answer pt1: {}", min_location);
    //     }
    // }

    pub mod part2 {

        use super::*;

        // #[test]
        // fn test_demo_input() {
        //     if !is_demo_mode() {
        //         return;
        //     }

        //     let input = get_input();
        //     let (seeds, maps) = parse_input(&input, parse_seed_part_2);
        //     assert_eq!(seeds.len(), 27);
        //     // Generates weird outputs
        //     let lowest_seed = 82;
        //     let correct_min_location = 46;
        //     let min_location = get_min_location(&vec![lowest_seed], &maps);
        //     assert_eq!(min_location, correct_min_location);

        //     let locations = map_in_chain(&seeds, &maps);
        //     let min_location = locations.iter().min().unwrap().clone();

        //     assert_eq!(min_location, 46);
        // }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            println!("Entering parse_input");
            let min_location = parse_input_2(&input);

            println!("Answer pt2: {}", min_location);
        }
    }
}
