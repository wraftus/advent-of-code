use std::fs;
use std::cmp::{min, max};

#[derive(Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn parse(coord_str: &str) -> Coord {
        let coords = coord_str.split(',').collect::<Vec<&str>>();
        assert!(coords.len() == 2, "Bad coordinate string!");

        let x = coords[0].parse::<usize>().expect("Failed to parse x coordinate!");
        let y = coords[1].parse::<usize>().expect("Failed to parse y coordinate!");
        Coord { x, y }
    }
}

#[derive(Clone)]
struct CaveSlice {
    min_x: usize,
    max_x: usize,
    max_y: usize,
    grid: Vec<Vec<bool>>,
}
impl CaveSlice {
    fn set_filled(&mut self, coord: Coord) {
        assert!(!self.is_out(&coord), "Tried filling an out of bounds position!");
        let coord = Coord {
            x: (coord.x - self.min_x),
            y: coord.y
        };
        self.grid[coord.x][coord.y] = true;
    }

    fn is_filled(&self, coord: &Coord) -> bool{
        if self.is_out(coord) { return false; }
        let coord = Coord {
            x: (coord.x - self.min_x),
            y: coord.y
        };
        return self.grid[coord.x][coord.y];
    }
    fn is_out(&self, coord: &Coord) -> bool {
        return (coord.x < self.min_x) || (coord.x > self.max_x) || (coord.y > self.max_y);
    }
}

fn read_input() -> CaveSlice {
    let file_contents = 
        fs::read_to_string("input/day14.txt")
        .expect("Failed to read input file!");

    // parse the rock formations from the input file
    let rocks_forms = file_contents.lines().map(|line| {
        line.split(" -> ").map(Coord::parse).collect::<Vec<Coord>>()
    }).collect::<Vec<Vec<Coord>>>();

    let min_coord = rocks_forms.iter().fold(Coord{x: 1000, y: 1000}, |min_coord, rock_bends| {
        rock_bends.iter().fold(min_coord, |min_coord, rock_bend| {
            let x = min(min_coord.x, rock_bend.x);
            let y = min(min_coord.y, rock_bend.y);
            return Coord{x, y};
        })
    });
    let max_coord = rocks_forms.iter().fold(Coord{x: 0, y: 0}, |min_coord, rock_bends| {
        rock_bends.iter().fold(min_coord, |min_coord, rock_bend| {
            let x = max(min_coord.x, rock_bend.x);
            let y = max(min_coord.y, rock_bend.y);
            return Coord{x, y};
        })
    });


    // fill the cave slice's grid according to the rock formations
    let grid = vec![vec![false; max_coord.y + 1]; (max_coord.x - min_coord.x) + 1]; 
    let mut cave_slice = CaveSlice {
        min_x: min_coord.x,
        max_x: max_coord.x,
        max_y: max_coord.y,
        grid
    };
    for rock_bends in rocks_forms {
        for bend_idx in 1..rock_bends.len() {
            let prev_bend = rock_bends.get(bend_idx - 1).unwrap();
            let curr_bend = rock_bends.get(bend_idx).unwrap();
            if curr_bend.x == prev_bend.x {
                for y in min(curr_bend.y, prev_bend.y)..=max(curr_bend.y, prev_bend.y) {
                    cave_slice.set_filled(Coord { x: curr_bend.x, y });
                }
            } else if curr_bend.y == prev_bend.y {
                for x in min(curr_bend.x, prev_bend.x)..=max(curr_bend.x, prev_bend.x) {
                    cave_slice.set_filled(Coord { x, y: curr_bend.y });
                }
            } else {
                unreachable!("Rock bend moves diagonally!");
            }
        }
    }
    return cave_slice;
}

// simulate a piece of sand falling in the cave slice
fn simulate_sand(cave_slice: &CaveSlice) -> Coord {
    let mut sand_pos = Coord { x: 500, y: 0 };
    loop {
        // check if the sand has fallen out of bounds
        if cave_slice.is_out(&sand_pos) {
            return sand_pos;
        }

        // check if the spot below is open
        let coord_below = Coord { x: sand_pos.x, y: (sand_pos.y + 1) };
        if !cave_slice.is_filled(&coord_below) {
            sand_pos = coord_below;
            continue;
        }
        
        // check if the spot to the bottom left or right is open
        let coord_left = Coord { x: (sand_pos.x - 1), y: (sand_pos.y + 1) };
        if !cave_slice.is_filled(&coord_left) {
            sand_pos = coord_left;
            continue;
        }
        let coord_right = Coord { x: (sand_pos.x + 1), y: (sand_pos.y + 1) };
        if !cave_slice.is_filled(&coord_right) {
            sand_pos = coord_right;
            continue;
        }

        // this piece of sand can no longer move, so we return
        return sand_pos;
    }
}

fn puzzle_one(cave_slice: &CaveSlice) -> usize {
    // create a copy of the cave slice
    let mut cave_slice = cave_slice.clone();

    // keep filling sand into the cave
    let mut sand_count = 0;
    loop {
        // simulate the sand, check if it went out of bounds
        let sand_pos = simulate_sand(&cave_slice);
        if cave_slice.is_out(&sand_pos) {
            return sand_count;
        }

        // fill the cave slice with the sand 
        cave_slice.set_filled(sand_pos);
        sand_count += 1;
    }
}

fn puzzle_two(cave_slice: &CaveSlice) -> usize {
    // copy the cave slice adding the floor
    let new_grid = cave_slice.grid.iter().fold(Vec::new(), |mut new_grid, vert_slice| {
        let mut vert_slice = vert_slice.clone();
        vert_slice.push(false);
        vert_slice.push(true);
        new_grid.push(vert_slice);
        return new_grid;
    });
    let mut cave_slice = CaveSlice{
        min_x: cave_slice.min_x,
        max_x: cave_slice.max_x,
        max_y: cave_slice.max_y + 2,
        grid: new_grid
    };

    // keep filling sand into the cave
    let mut sand_count = 0;
    loop {
        // simulate the sand, checking if we are done
        let sand_pos = simulate_sand(&cave_slice);
        if (sand_pos.x == 500) && (sand_pos.y == 0) {
            return sand_count + 1;
        }

        // see what happens to this new sand piece
        if cave_slice.is_out(&sand_pos) {
            // sand fell out, push it down to the floor
            let mut vert_slice = vec![false; cave_slice.max_y + 1];
            vert_slice[cave_slice.max_y] = true;
            vert_slice[cave_slice.max_y - 1] = true;

            if sand_pos.x < cave_slice.min_x {
                cave_slice.grid.insert(0, vert_slice);
                cave_slice.min_x -= 1;
            } else if sand_pos.x > cave_slice.max_x {
                cave_slice.grid.push(vert_slice);
                cave_slice.max_x += 1;
            } else {
                unreachable!("Sand fell below the floor!");
            }
        } else {
            // sand has stopped moving, add it to the grid
            cave_slice.set_filled(sand_pos);
        }

        // increment sand count
        sand_count += 1;
    }
}

fn main() {
    let cave_slice = read_input();

    println!("Puzzle 1: {}", puzzle_one(&cave_slice));
    println!("Puzzle 2: {}", puzzle_two(&cave_slice));
}