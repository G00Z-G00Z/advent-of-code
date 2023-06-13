use std::fs;

use camp_cleanup::get_ranges;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut overlaps = 0;

    for lines in input.lines() {
        let ranges = get_ranges(&lines);
        let r1 = ranges.get(0).unwrap();
        let r2 = ranges.get(1).unwrap();

        match (r1.overlaps(r2), r2.overlaps(r1)) {
            (false, false) => {}
            (_, _) => {
                overlaps += 1;
            }
        }
    }

    println!("Overlaps: {}", overlaps);
}
