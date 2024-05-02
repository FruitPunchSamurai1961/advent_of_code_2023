use std::cmp::min;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct RangeSet {
    ranges: Vec<(u64, u64, u64)>,
}

impl RangeSet {
    fn new() -> Self {
        RangeSet { ranges: Vec::new() }
    }

    fn insert(&mut self, source_start: u64, dest_start: u64, range_length: u64) {
        self.ranges.push((source_start, dest_start, range_length));
    }

    fn get(&self, start: u64, end: u64) -> Vec<(u64, u64)> {
        let mut new_ranges = Vec::new();

        for &(source_start, dest_start, range_length) in &self.ranges {
            let source_end = source_start + range_length - 1;
            let dest_end = dest_start + range_length - 1;
            if source_start <= start && start <= source_end && end > source_end {
                //Case where start is in range but end is not
                let new_start = dest_start + (start - source_start);
                new_ranges.push((new_start, dest_end));
                new_ranges.extend(self.get(source_end + 1, end));
                return new_ranges;
            } else if source_start > start && source_start <= end && end <= source_end {
                //Case where start is not in range but end is
                let new_end = dest_start + (end - source_start);
                new_ranges.push((dest_start, new_end));
                new_ranges.extend(self.get(start, source_start - 1));
                return new_ranges;
            } else if source_start <= start && end <= source_end {
                //Case where end and start are all in range
                let new_start = dest_start + (start - source_start);
                let new_end = dest_start + (end - source_start);
                new_ranges.push((new_start, new_end));
                return new_ranges;
            }
        }
        // Case 4: No overlapping ranges found, return the input range
        new_ranges.push((start, end));
        new_ranges
    }
}

pub fn solve(input: &str) {
    println!("Solving Day 5 problems...");
    let (part_1_seeds, part_2_seeds, sections) = parse_input(input);

    let p1_solution = get_lowest_location_from_seeds(&part_1_seeds, &sections);
    println!("The answer to the first gold star for day 5 is: {}", p1_solution);

    let p2_solution = get_lowest_location_from_seeds(&part_2_seeds, &sections);
    println!("The answer to the second gold star for day 5 is: {}", p2_solution);
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<(u64, u64)>, HashMap<String, RangeSet>) {
    let mut part_1_seeds = Vec::new();
    let mut part_2_seeds = Vec::new();
    let mut sections = HashMap::new();
    let mut current_section = String::new();

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("seeds:") {
            part_1_seeds = parse_part_1_seeds(line);
            part_2_seeds = parse_part_2_seeds(line);
        } else if line.ends_with("map:") {
            if !current_section.is_empty() {
                let (key, value) = parse_section(&current_section.clone());
                sections.insert(key, value);
                current_section.clear();
            }
            current_section = String::from(line);
        } else {
            current_section.push_str("\n");
            current_section.push_str(line);
        }
    }
    if !current_section.is_empty() {
        let (key, value) = parse_section(&current_section.clone());
        sections.insert(key, value);
    }

    (part_1_seeds, part_2_seeds, sections)
}

fn parse_part_1_seeds(input: &str) -> Vec<(u64, u64)> {
    let mut seeds = Vec::new();
    let values = input.split_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap());
    for val in values {
        seeds.push((val, val));
    }
    seeds
}

fn parse_part_2_seeds(input: &str) -> Vec<(u64, u64)> {
    let mut seeds = Vec::new();
    let mut values = input.split_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap());
    while let (Some(val1), Some(val2)) = (values.next(), values.next()) {
        seeds.push((val1, val1 + val2));
    }
    seeds
}


fn parse_section(section: &str) -> (String, RangeSet) {
    let mut section_name = String::new();
    let mut parsed_section = RangeSet::new();
    let mut lines = section.lines();

    if let Some(first_line) = lines.next() {
        section_name = first_line.trim_end_matches(" map:").parse().unwrap();
    }

    for line in lines {
        let mut values = line.split_whitespace();
        if let (Some(dest_start), Some(source_start), Some(range_length)) = (values.next(), values.next(), values.next()) {
            if let (Ok(dest_start), Ok(source_start), Ok(range_length)) = (dest_start.parse::<u64>(), source_start.parse::<u64>(), range_length.parse::<u64>()) {
                parsed_section.insert(source_start, dest_start, range_length);
            }
        }
    }

    (section_name, parsed_section)
}

fn get_lowest_location_from_seeds(seeds: &Vec<(u64, u64)>, sections: &HashMap<String, RangeSet>) -> u64 {
    let soil_range = get_new_ranges(seeds, sections.get("seed-to-soil").unwrap());
    let fertilizer_range = get_new_ranges(&soil_range, sections.get("soil-to-fertilizer").unwrap());
    let water_range = get_new_ranges(&fertilizer_range, sections.get("fertilizer-to-water").unwrap());
    let light_range = get_new_ranges(&water_range, sections.get("water-to-light").unwrap());
    let temperature_range = get_new_ranges(&light_range, sections.get("light-to-temperature").unwrap());
    let humidity_range = get_new_ranges(&temperature_range, sections.get("temperature-to-humidity").unwrap());
    let location_range = get_new_ranges(&humidity_range, sections.get("humidity-to-location").unwrap());
    return get_lowest_from_ranges(&location_range);
}

fn get_lowest_from_ranges(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut curr_lowest = u64::MAX;

    for &(val, _) in ranges {
        curr_lowest = min(curr_lowest, val);
    }
    curr_lowest
}


fn get_new_ranges(curr_range: &Vec<(u64, u64)>, range_set: &RangeSet) -> Vec<(u64, u64)> {
    let mut new_ranges = Vec::new();
    for (start, end) in curr_range {
        new_ranges.extend(range_set.get(*start, *end));
    }
    new_ranges
}
