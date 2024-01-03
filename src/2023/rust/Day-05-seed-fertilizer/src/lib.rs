#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn new(min: u64, max: u64) -> Self {
        assert!(min <= max, "min must be less than or equal to max");
        Self { min, max }
    }

    fn from_range(min: u64, range: usize) -> Self {
        let max = min + range as u64 - 1;
        Self::new(min, max)
    }

    fn range(&self) -> u64 {
        self.max - self.min + 1
    }

    fn contains(&self, value: u64) -> bool {
        self.min <= value && value <= self.max
    }
}

#[derive(Debug)]
struct RangedMap {
    input: u64,
    output: u64,
    range: u64,
}

impl RangedMap {
    fn new(input: u64, output: u64, range: u64) -> Self {
        Self {
            input,
            output,
            range,
        }
    }

    fn map(&self, value: u64) -> u64 {
        if !self.contains(value) {
            return value;
        }

        let value_idx = value - self.input;
        let new_value = self.output + value_idx;
        new_value
    }

    fn contains(&self, value: u64) -> bool {
        self.input <= value && value <= self.input + self.range
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

fn parse_input(input: &str) -> (SeedList, Vec<XToYMap>) {
    let mut maps = Vec::new();

    let mut lines = input.split("\n\n");

    // First line is the seed list:
    // E.g. : "seeds: 79 14 55 13"
    let mut seeds = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<SeedList>();

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

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    pub mod part1 {

        use super::*;

        #[test]
        fn test_ranges() {
            let x_to_y_map =
                XToYMap::new(vec![RangedMap::new(50, 98, 2), RangedMap::new(52, 50, 48)]);

            let value = 81;

            let new_value = x_to_y_map.map(value);

            assert_eq!(new_value, 79);
        }

        // #[test]
        // fn test_demo_input() {
        //     if !is_demo_mode() {
        //         return;
        //     }

        //     let input = get_input();
        //     let (seeds, maps) = parse_input(&input);
        //     println!("Seeds: {:?}", seeds);
        //     let locations = map_in_chain(&seeds, &maps);
        //     let min_location = locations.iter().min().unwrap().clone();
        //     println!("Locations: {:?}", locations);
        //     println!("Min: {}", min_location);
        //     assert_eq!(min_location, 35);
        // }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();
            let (seeds, maps) = parse_input(&input);
            let min_location = get_min_location(&seeds, &maps);

            println!("Answer pt1: {}", min_location);
        }
    }

    // pub mod part2 {

    //     use super::*;

    //     #[test]
    //     fn test_demo_input() {
    //         if !is_demo_mode() {
    //             return;
    //         }

    //         let input = get_input();

    //         assert_eq!(input, "hey");
    //     }

    //     #[test]
    //     fn test_input() {
    //         if is_demo_mode() {
    //             return;
    //         }

    //         let input = get_input();

    //         println!("Answer pt2: {}", input);
    //     }
    // }
}
