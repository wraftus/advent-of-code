use std::fs;
use std::cmp::{max, min};
use regex::Regex;

struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

struct Floor {
    x_size: usize,
    y_size: usize,
    locations: Vec<Vec<usize>>
}
impl Floor {
    fn new(x_size: usize, y_size: usize) -> Floor {
        Floor { x_size, y_size, locations: vec![vec![0; x_size]; y_size] }
    }

    fn reset(&mut self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                self.locations[y][x] = 0;
            }
        }
    }

    fn apply_vent(&mut self, vent: &Vent) {
        let x_diff = ((vent.x1 as isize) - (vent.x2 as isize)).abs() as usize;
        let y_diff = ((vent.y1 as isize) - (vent.y2 as isize)).abs() as usize;

        if x_diff == 0 {
            let y_start = min(vent.y1, vent.y2);
            for y_rel in 0..(y_diff + 1) {
                self.locations[y_start + y_rel][vent.x1] += 1;
            }
        }
        else if y_diff == 0 {
            let x_start = min(vent.x1, vent.x2);
            for x_rel in 0..(x_diff + 1) {
                self.locations[vent.y1][x_start + x_rel] += 1;
            }
        }
        else if x_diff == y_diff {
            let x_sign: isize = if vent.x1 < vent.x2 { 1 } else { -1 };
            let y_sign: isize = if vent.y1 < vent.y2 { 1 } else { -1 };
            for pos_rel in 0..(x_diff + 1) {
                let x: isize = (vent.x1 as isize) + x_sign*(pos_rel as isize);
                let y: isize = (vent.y1 as isize) + y_sign*(pos_rel as isize);
                self.locations[y as usize][x as usize] += 1;
            }
        }
    }

    fn num_overlapping(&self) -> usize {
        let mut counter = 0;
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                if self.locations[y][x] >= 2 { counter += 1; }
            }
        }
        return counter;
    }
}

fn parse_input() -> Vec<Vent> {
    let file_contents: String =
        fs::read_to_string("input/day05.txt")
        .expect("Failed to read input file!");

    let line_re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    file_contents.lines().map(|line_str: &str| {
        let captures = line_re.captures(line_str).expect("Failed to regex line!");
        assert!(captures.len() == 4+1, "Line has incorrect number of captures!");
        Vent {
            x1: captures.get(1).unwrap().as_str().parse().unwrap(),
            y1: captures.get(2).unwrap().as_str().parse().unwrap(),
            x2: captures.get(3).unwrap().as_str().parse().unwrap(),
            y2: captures.get(4).unwrap().as_str().parse().unwrap(),
        }
    }).collect()
}

fn puzzle_one(vents: &Vec<Vent>, floor: &mut Floor) -> usize {
    for vent in vents.iter() {
        if vent.x1 == vent.x2 || vent.y1 == vent.y2 {
            floor.apply_vent(&vent);
        }
    }
    return floor.num_overlapping();
}

fn puzzle_two(vents: &Vec<Vent>, floor: &mut Floor) -> usize {
    for vent in vents.iter() { floor.apply_vent(&vent); }
    return floor.num_overlapping();
}

fn get_floor_size(vents: &Vec<Vent>) -> (usize, usize) {
    let first_vent = &vents[0];
    let mut max_x = max(first_vent.x1, first_vent.x2);
    let mut max_y = max(first_vent.y1, first_vent.y2);

    for vent in vents[1..].iter() {
        if vent.x1 > max_x { max_x = vent.x1; }
        if vent.x2 > max_x { max_x = vent.x2; }
        if vent.y1 > max_y { max_y = vent.y1; }
        if vent.y2 > max_y { max_y = vent.y2; }
    }

    return (max_x + 1, max_y + 1);
}

fn main() {
    let vents = parse_input();

    let (x_size, y_size) = get_floor_size(&vents);
    let mut floor = Floor::new(x_size, y_size);

    println!("Puzzle 1: {}", puzzle_one(&vents, &mut floor));
    floor.reset();
    println!("Puzzle 2: {}", puzzle_two(&vents, &mut floor));
}
