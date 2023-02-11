use std::fs;
use std::cmp::{min, max};
use regex::Regex;

struct Coord {
    x : isize,
    y : isize,
}
impl Coord {
    fn parse(x_str: &str, y_str: &str) -> Coord {
        let x = x_str.parse::<isize>().expect("Failed to parse x string!");
        let y = y_str.parse::<isize>().expect("Failed to parse y string!");
        Coord { x, y }
    }
    fn l1_dist(p1: &Coord, p2: &Coord) -> isize {
        return (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
    }
}

#[derive(Clone, Debug)]
struct Range {
    min : isize,
    max : isize
}
impl Range {
    fn union(r1: &Range, r2: &Range) -> Option<Range> {
        if r1.max < r2.min { return None; }
        if r2.max < r1.min { return None; }

        let max = max(r1.max, r2.max);
        let min = min(r1.min, r2.min);
        Some(Range { min, max })
    }

    fn width(&self) -> usize {
        return (self.max - self.min) as usize;
    }
}

fn read_input() -> Vec<(Coord, Coord)> {
    let file_contents = 
        fs::read_to_string("input/day15.txt")
        .expect("Failed to read input file!");

    let re = Regex::new(
        r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
    ).unwrap();
    file_contents.lines().map(|line| {
        let captures = re.captures(line).expect("Failed to regex line!");

        let sensor = Coord::parse(
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str()
        );
        let beacon = Coord::parse(
            captures.get(3).unwrap().as_str(),
            captures.get(4).unwrap().as_str()
        );
        (sensor, beacon)
    }).collect::<Vec<(Coord, Coord)>>()
}

fn empty_ranges_at_height(coords: &Vec<(Coord, Coord)>, height: isize) -> Vec<Range> {
    let mut empty_ranges: Vec<Range> = Vec::new();
    for (sensor, beacon) in coords.iter() {
        let proj_radius = max(0, Coord::l1_dist(sensor, beacon) - (sensor.y - height).abs());
        if proj_radius == 0 { continue; }

        let mut added_to_list = false;
        let proj_range = Range { min: (sensor.x - proj_radius), max: (sensor.x + proj_radius) };
        for idx in 0..empty_ranges.len() {
            let other_range = empty_ranges.get(idx).unwrap();
            if let Some(merged_range) = Range::union(&proj_range, other_range) {
                match empty_ranges.get(idx + 1) {
                    Some(next_range) => {
                        match Range::union(&merged_range, next_range) {
                            Some(new_merged_range) => {
                                empty_ranges[idx] = new_merged_range;
                                empty_ranges.remove(idx + 1);
                            }
                            None => { empty_ranges[idx] = merged_range; }
                        }
                    },
                    None => { empty_ranges[idx] = merged_range; }
                }
                added_to_list = true;
                break;
            } else if proj_range.max < other_range.min {
                empty_ranges.insert(idx, proj_range.clone());
                added_to_list = true;
                break;
            }
        }
        if !added_to_list {
            empty_ranges.push(proj_range.clone());
        }
    }
    return empty_ranges;
}

fn puzzle_one(coords: &Vec<(Coord, Coord)>) -> usize {
    const CHECK_ROW : isize = 2000000;

    empty_ranges_at_height(coords, CHECK_ROW)
        .iter().fold(0, |count, range| {
            count + range.width()
        })
}

fn puzzle_two(coords: &Vec<(Coord, Coord)>) -> usize {
    const WINDOW_SIZE: usize = 4000000;

    for height in 0..WINDOW_SIZE {
        let proj_ranges = empty_ranges_at_height(coords, height as isize);
        if proj_ranges.len() == 1 { continue; }

        for proj_range in proj_ranges.iter() {
            if 0 <= proj_range.max && proj_range.max <= WINDOW_SIZE as isize {
                let distress = Coord { x: proj_range.max + 1, y: height as isize };
                return (distress.x * 4000000 + distress.y) as usize;
            }
        }
    }
    unreachable!("Failed to find the distress signal!");
}

fn main() {
    let coords = read_input();

    println!("Puzzle 1: {}", puzzle_one(&coords));
    println!("Puzzle 2: {}", puzzle_two(&coords));
}