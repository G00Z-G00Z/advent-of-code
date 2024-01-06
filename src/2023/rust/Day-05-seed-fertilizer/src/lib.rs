#![allow(dead_code, non_snake_case)]
use std::ops::Range;

#[derive(Debug)]
struct RangedMap {
    range: Range<u64>,
    output_start: u64,
}

impl RangedMap {
    fn new(input: u64, output: u64, range: u64) -> Self {
        Self {
            range: input..(input + range),
            output_start: output,
        }
    }

    fn map(&self, value: u64) -> u64 {
        let value_idx = value - self.range.start;
        let new_value = self.output_start + value_idx;
        new_value
    }

    fn contains(&self, value: u64) -> bool {
        self.range.contains(&value)
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

fn parse_seed_part_2(input: &str) -> SeedList {
    // E.g. : "seeds: 79 14 55 13"
    // Where  "seeds: init1 range1 init2 range2 ..."
    // The range is inclusive (E.g. 1 3 = 1, 2, 3)
    input
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .flat_map(|pair| pair[0]..=(pair[1] + pair[0] - 1))
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

        maps.push(map_ranges);
    }

    let initial_seed_ranges = seeds
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|pair| pair[0]..(pair[1] + pair[0]))
        .collect::<Vec<Range<u64>>>();

    println!("Initial seed ranges: {:?}", initial_seed_ranges);

    // Update the seed ranges
    // Find the limits of the seed range and the map, and only
    // update the bounds of the ranges
    let mut new_seed_ranges = initial_seed_ranges.clone();

    for map in maps.iter() {
        // Transform just one map
        let seed_ranges = new_seed_ranges.clone();
        new_seed_ranges.clear();

        for seed_range in seed_ranges {
            let mut check_intersections = vec![seed_range];
            let mut new_intersections = vec![];

            while !check_intersections.is_empty() {
                // Get an intersection
                let intersection = check_intersections.pop().unwrap();
                let mut mapped = false;

                // This only maps one intersection at the time
                for submap in map.iter() {
                    // Check if the intersection is outside  the map
                    // |  |        -> intersection
                    //        | |  -> map
                    //
                    //        | |  -> intersection
                    // |  |        -> map
                    //    |     |  -> map
                    // |  |        -> intersection
                    //    |     |  -> intersection
                    // |  |        -> map
                    if intersection.start >= submap.range.end
                        || intersection.end <= submap.range.start
                    {
                        continue;
                    }

                    // Some intersection
                    mapped = true;
                    match (
                        intersection.start >= submap.range.start,
                        intersection.end <= submap.range.end,
                    ) {
                        (true, true) => {
                            // The map is inside the intersection
                            //    | |      -> intersection
                            //  |     |   -> map
                            //  or
                            //  |     |    -> intersection
                            //  |     |   -> map
                            new_intersections
                                .push(submap.map(intersection.start)..submap.map(intersection.end));
                        }
                        (true, false) => {
                            // Submap has a division on the right
                            //    |    |   -> intersection
                            //  |     |   -> map
                            //  or
                            //  |      |   -> intersection
                            //  |     |   -> map

                            let inside =
                                submap.map(intersection.start)..submap.map(submap.range.end);
                            new_intersections.push(inside);

                            let outside = submap.range.end..intersection.end;
                            check_intersections.push(outside);
                        }
                        (false, true) => {
                            // Intersection has division on the left
                            // |     |     -> intersection
                            //  |     |   -> map
                            //  or
                            // |      |    -> intersection
                            //  |     |   -> map

                            let inside =
                                submap.map(submap.range.start)..submap.map(intersection.end);
                            new_intersections.push(inside);

                            let outside = intersection.start..submap.range.start;
                            check_intersections.push(outside);
                        }
                        (false, false) => {
                            // Map is completely inside the intersection
                            // |       |   -> intersection
                            //  |     |   -> map
                            let left = intersection.start..submap.range.start;
                            let inside =
                                submap.map(submap.range.start)..submap.map(submap.range.end);
                            let outside = submap.range.end..intersection.end;

                            check_intersections.push(left);
                            new_intersections.push(inside);
                            check_intersections.push(outside);
                        }
                    }
                    break;
                }

                if !mapped {
                    new_intersections.push(intersection);
                }
            }

            new_seed_ranges.extend(new_intersections);
        }
    }

    new_seed_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap() as u64
}

const CORRECT_LOCATIONS_PART_1: [u64; 4] = [82, 43, 86, 35];
const CORRECT_MIN_LOCATION_PART_1: u64 = 13;

#[cfg(test)]
mod tests {
    use super::*;
    use utility_2022::{get_input, is_demo_mode};

    pub mod part2 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();
            let min_location = parse_input_2(&input);
            println!("Answer pt2: {}", min_location);

            assert_eq!(min_location, 46);
        }

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
