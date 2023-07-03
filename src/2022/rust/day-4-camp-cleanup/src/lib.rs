#[derive(Debug)]
pub struct Range(usize, usize);

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        !(self.0 > other.1 || other.0 > self.1)
    }
}

pub fn get_ranges(range_text: &str) -> Vec<Range> {
    let ranges = range_text.split(",").collect::<Vec<&str>>();

    let mut range_vec = Vec::new();

    for range in ranges.iter() {
        let range = range.trim();
        let range = range.split("-").collect::<Vec<&str>>();
        let range = Range(
            range[0].parse::<usize>().unwrap(),
            range[1].parse::<usize>().unwrap(),
        );
        range_vec.push(range);
    }
    range_vec
}
