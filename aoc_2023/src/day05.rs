use std::fs;
use std::str::Lines;
use std::cmp::{min, max, Ordering};

struct RangeMap {
    dest_start: usize,
    src_start:  usize,
    length:     usize
}
impl RangeMap {
    fn src_end(&self) -> usize {
        return self.src_start + self.length - 1;
    }

    fn source_to_dest(&self, source: usize) -> Option<usize> {
        if source <  self.src_start { return None; }
        if source >= self.src_end() { return None; }

        let rel_idx = source - self.src_start;
        return Some(self.dest_start + rel_idx)
    }

    fn sort_key(range_map1: &RangeMap, range_map2: &RangeMap) -> Ordering {
        range_map1.src_start.cmp(&range_map2.src_start)
    }
}

struct Almanac {
    seeds:          Vec<usize>,
    seed_to_soil:   Vec<RangeMap>,
    soil_to_fert:   Vec<RangeMap>,
    fert_to_water:  Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temp:  Vec<RangeMap>,
    temp_to_humid:  Vec<RangeMap>,
    humid_to_loc:   Vec<RangeMap>
}
impl Almanac {
    fn all_maps(&self) -> [&Vec<RangeMap>; 7] {
        return [
            &self.seed_to_soil,
            &self.soil_to_fert,
            &self.fert_to_water,
            &self.water_to_light,
            &self.light_to_temp,
            &self.temp_to_humid,
            &self.humid_to_loc
        ]
    }
    fn all_maps_mut(&mut self) -> [&mut Vec<RangeMap>; 7] {
        return [
            &mut self.seed_to_soil,
            &mut self.soil_to_fert,
            &mut self.fert_to_water,
            &mut self.water_to_light,
            &mut self.light_to_temp,
            &mut self.temp_to_humid,
            &mut self.humid_to_loc
        ]
    }
}

fn parse_range_maps(file_lines: &mut Lines) -> Vec<RangeMap> {
    let mut range_maps: Vec<RangeMap> = Vec::new();
    while let Some(range_map_line) = file_lines.next() {
        if range_map_line.is_empty() { break; }
        let map_data_strs: Vec<&str> = range_map_line.split(" ").collect();
        assert!(map_data_strs.len() == 3, "Malfored range map line!");

        range_maps.push(RangeMap {
            dest_start: map_data_strs.get(0).unwrap().parse().expect("Invalid destination start!"),
            src_start:  map_data_strs.get(1).unwrap().parse().expect("Invalid source start!"),
            length:     map_data_strs.get(2).unwrap().parse().expect("Invalid range map length!")
        });
    }
    return range_maps;
}

fn read_input() -> Almanac {
    let file_contents =
        fs::read_to_string("input/day05.txt")
        .expect("Failed to read input file!");
    let mut file_lines = file_contents.lines();

    let seeds_line = file_lines.next().expect("Input file is empty!");
    assert!(seeds_line.starts_with("seeds: "), "Malformed seeds line!");
    let seeds: Vec<usize> = seeds_line
        .trim_start_matches("seeds: ").split_whitespace()
        .map(|num_str: &str| num_str.parse().unwrap()).collect();

    assert!(file_lines.next().expect("File ends early!").is_empty(), "Missing empty line!");
    assert!(file_lines.next().expect("File ends early!") == "seed-to-soil map:",
        "Missing seed to soil header line!");
    let seed_to_soil = parse_range_maps(&mut file_lines);

    assert!(file_lines.next().expect("File ends early!") == "soil-to-fertilizer map:",
        "Missing soild to fertilizer header line!");
    let soil_to_fert = parse_range_maps(&mut file_lines);

    assert!(file_lines.next().expect("File ends early!") == "fertilizer-to-water map:",
        "Missing fertilizer to water header line!");
    let fert_to_water = parse_range_maps(&mut file_lines);

    assert!(file_lines.next().expect("File ends early!") == "water-to-light map:",
        "Missing water to light header line!");
    let water_to_light = parse_range_maps(&mut file_lines);

    assert!(file_lines.next().expect("File ends early!") == "light-to-temperature map:",
        "Missing light to temperature header line!");
    let light_to_temp = parse_range_maps(&mut file_lines);

    assert!(file_lines.next().expect("File ends early!") == "temperature-to-humidity map:",
        "Missing temperature to humidity header line!");
    let temp_to_humid = parse_range_maps(&mut file_lines);

    assert!(file_lines.next().expect("File ends early!") == "humidity-to-location map:",
        "Missing humidity to location header line!");
    let humid_to_loc = parse_range_maps(&mut file_lines);

    Almanac {
        seeds, seed_to_soil, soil_to_fert, fert_to_water, water_to_light, light_to_temp,
        temp_to_humid, humid_to_loc
    }
}

fn perform_value_mapping(source: usize, range_maps: &Vec<RangeMap>) -> usize {
    for range_map in range_maps.iter() {
        if let Some(destination) = range_map.source_to_dest(source) {
            return destination;
        }
    }
    return source;
}

fn puzzle_one(almanac: &Almanac) -> usize {
    almanac.seeds.iter().map(|seed|
        almanac.all_maps().iter()
            .fold(*seed, |source, range_map| perform_value_mapping(source, range_map))
    ).min().unwrap()
}

#[derive(Clone)]
struct Range {
    start:  usize,
    length: usize
}

// this function assumes that the range_maps are sorted, else it does not work properly
fn perform_range_mapping(source_range: &Range, range_maps: &Vec<RangeMap>) -> Vec<Range>{
    let mut destination_ranges: Vec<Range> = Vec::new();

    let mut range_map_idx = 0;
    let mut remaining_range = source_range.clone();
    while remaining_range.length > 0 {
        let cur_range_map = &range_maps[range_map_idx];

        // remaining range start before the current range map
        if remaining_range.start < cur_range_map.src_start {
            let range_taken = cur_range_map.src_start - remaining_range.start;
            destination_ranges.push(
                Range {
                    start:  remaining_range.start,
                    length: min(remaining_range.length, range_taken)
                }
            );

            let range_difference = (remaining_range.length as isize) - (range_taken as isize);
            remaining_range.start  = cur_range_map.src_start;
            remaining_range.length = max(0, range_difference) as usize;
        }

        // remaining range stars within the current range map
        else if remaining_range.start <= cur_range_map.src_end() {
            let range_taken = cur_range_map.src_end() - remaining_range.start + 1;
            let rel_offset  = remaining_range.start - cur_range_map.src_start;
            destination_ranges.push(
                Range {
                    start:  cur_range_map.dest_start + rel_offset,
                    length: min(remaining_range.length, range_taken)
                }
            );

            let range_difference = (remaining_range.length as isize) - (range_taken as isize);
            remaining_range.start  = cur_range_map.src_end() + 1;
            remaining_range.length = max(0, range_difference) as usize;
        }

        // remaining range starts past the current range map
        else {
            range_map_idx += 1;
            if range_map_idx == range_maps.len() {
                destination_ranges.push(remaining_range.clone());
                remaining_range.length = 0;
            }
        }
    }

    return destination_ranges;
}

fn puzzle_two(almanac: &mut Almanac) -> usize {
    for range_map in almanac.all_maps_mut().iter_mut() {
        range_map.sort_by(RangeMap::sort_key);
    }

    let seed_ranges: Vec<Range> = (0..almanac.seeds.len()).step_by(2).map(|start_idx|
        Range {
            start:  almanac.seeds[start_idx],
            length: almanac.seeds[start_idx + 1]
        }
    ).collect();

    almanac.all_maps().iter().fold(seed_ranges, |src_ranges, range_maps| {
        src_ranges.iter().fold(Vec::new(), |mut dest_ranges, src_range| {
            dest_ranges.extend(perform_range_mapping(src_range, range_maps));
            return dest_ranges;
        })
    }).iter().map(|range| range.start).min().unwrap()
}

fn main() {
    let mut almanac = read_input();

    println!("Puzzle 1: {}", puzzle_one(&mut almanac));
    println!("Puzzle 2: {}", puzzle_two(&mut almanac));
}
