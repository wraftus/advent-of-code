use std::fs;
use std::cmp::{min, max};

struct Galaxy {
    x: usize,
    y: usize
}

fn read_input() -> Vec<Galaxy> {
    let file_contents =
        fs::read_to_string("input/day11.txt")
        .expect("Failed to read input file!");

    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (line_idx, line) in file_contents.lines().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push(Galaxy { x: char_idx, y: line_idx });
            }
        }
    }

    return galaxies;
}

const UNIVERSE_WIDTH: usize = 140;

fn puzzle_one(galaxies: &Vec<Galaxy>) -> usize {
    let mut is_col_empty = [true; UNIVERSE_WIDTH];
    let mut is_row_empty = [true; UNIVERSE_WIDTH];
    for galaxy in galaxies {
        is_col_empty[galaxy.x] = false;
        is_row_empty[galaxy.y] = false;
    }

    let mut counter = 0;
    for (idx, galaxy_i) in galaxies.iter().enumerate() {
        for galaxy_j in galaxies.iter().skip(idx + 1) {
            let min_x: usize = min(galaxy_i.x, galaxy_j.x);
            let min_y: usize = min(galaxy_i.y, galaxy_j.y);
            let max_x: usize = max(galaxy_i.x, galaxy_j.x);
            let max_y: usize = max(galaxy_i.y, galaxy_j.y);

            counter += (max_x - min_x) + (max_y - min_y);
            for x in (min_x + 1)..max_x {
                if is_col_empty[x] { counter += 1; }
            }
            for y in (min_y + 1)..max_y {
                if is_row_empty[y] { counter += 1; }
            }
        }
    }
    return counter;
}

fn puzzle_two(galaxies: &Vec<Galaxy>) -> usize {
    let mut is_col_empty = [true; UNIVERSE_WIDTH];
    let mut is_row_empty = [true; UNIVERSE_WIDTH];
    for galaxy in galaxies {
        is_col_empty[galaxy.x] = false;
        is_row_empty[galaxy.y] = false;
    }

    let mut counter = 0;
    for (idx, galaxy_i) in galaxies.iter().enumerate() {
        for galaxy_j in galaxies.iter().skip(idx + 1) {
            let min_x: usize = min(galaxy_i.x, galaxy_j.x);
            let min_y: usize = min(galaxy_i.y, galaxy_j.y);
            let max_x: usize = max(galaxy_i.x, galaxy_j.x);
            let max_y: usize = max(galaxy_i.y, galaxy_j.y);

            counter += (max_x - min_x) + (max_y - min_y);
            for x in (min_x + 1)..max_x {
                if is_col_empty[x] { counter += 999999; }
            }
            for y in (min_y + 1)..max_y {
                if is_row_empty[y] { counter += 999999; }
            }
        }
    }
    return counter;
}


fn main() {
    let galaxies = read_input();

    println!("Puzzle 1: {}", puzzle_one(&galaxies));
    println!("Puzzle 1: {}", puzzle_two(&galaxies));
}