use std::fs;
use std::collections::{HashSet, HashMap};

enum Square {
    Start,
    Height(usize),
    End
}
impl Square {
    fn height(&self) -> usize {
        match self {
            Square::Start     => 0,
            Square::Height(h) => *h,
            Square::End       => 25
        }
    }
}
type Grid = Vec<Vec<Square>>;
type Pos = (usize, usize);

enum Moves {
    Up,
    Down,
    Left,
    Right
}
impl Moves {
    fn try_move(&self, curr_pos: (usize, usize), grid: &Grid) -> Option<Pos> {
        // check that the move remains in bounds
        let move_pos = match self {
            Moves::Up => {
                if curr_pos.1 == 0 { return None; }
                (curr_pos.0, curr_pos.1 - 1)
            },
            Moves::Down  => {
                if curr_pos.1 + 1 == grid[curr_pos.0].len() { return None; }
                (curr_pos.0, curr_pos.1 + 1)
            },
            Moves::Left  => {
                if curr_pos.0 == 0 { return None; }
                (curr_pos.0 - 1, curr_pos.1)
            },
            Moves::Right => {
                if curr_pos.0 + 1 == grid.len() { return None; }
                (curr_pos.0 + 1, curr_pos.1)
            },
        };

        // check that the heights allow a move
        let curr_sqr = &grid[curr_pos.0][curr_pos.1];
        let move_sqr = &grid[move_pos.0][move_pos.1];
        if (curr_sqr.height() + 1) < move_sqr.height() { return None; }
        return Some(move_pos);
    }
}

fn read_input() -> Grid {
    let file_contents = 
        fs::read_to_string("input/day12.txt")
        .expect("Failed to read input file!");

    file_contents.lines()
        .map(|line| line.chars().map(|square| {
            match square {
                'S' => Square::Start,
                'E' => Square::End,
                _   => Square::Height((square as usize) - ('a' as usize))
            }
        }).collect::<Vec<Square>>()
    ).collect::<Grid>()
}

fn shortest_path(init_pos: Pos, grid: &Grid) -> Option<usize> {
    // set up visited hashset and to visit hash map
    let mut visited : HashSet<Pos> = HashSet::new();
    let mut to_visit: HashMap<Pos, usize> = HashMap::new();
    to_visit.insert(init_pos, 0);
    while to_visit.len() > 0 {
        // determine the next position to try
        let (curr_pos, curr_dist) = to_visit.iter().fold(None, |min_pair, (&pos, &dist)| {
            match min_pair {
                Some((min_pos, min_dist)) => {
                    if dist < min_dist {
                        Some((pos, dist))
                    } else {
                        Some((min_pos, min_dist))
                    }
                }
                None => Some((pos, dist)),
            }
        }).unwrap();
        to_visit.remove(&curr_pos);
        visited.insert(curr_pos);

        // check if we are done
        match &grid[curr_pos.0][curr_pos.1] {
            Square::End => { return Some(curr_dist); }
            _ => {}
        }

        // if not, try possible moves and add them to `to_visit`
        for to_move in [Moves::Up, Moves::Down, Moves::Right, Moves::Left] {
            if let Some(move_pos) = to_move.try_move(curr_pos, grid) {
                // make sure we have not already visited this place
                if visited.contains(&move_pos) { continue; }

                // check if we can get to this pos in a shorter path
                if let Some(other_dist) = to_visit.get(&move_pos) {
                    if *other_dist > curr_dist {
                        to_visit.insert(move_pos, curr_dist + 1);
                    }
                } else {
                    to_visit.insert(move_pos, curr_dist + 1);
                }
            }
        }
    }
    None
}

fn puzzle_one(grid: &Grid) -> usize {
    // determine the starting pos
    let mut init_pos: Option<Pos> = None;
    for (idx, row) in grid.iter().enumerate() {
        for (jdx, sqr) in row.iter().enumerate() {
            match sqr {
                Square::Start => {
                    init_pos = Some((idx, jdx));
                    break;
                }
                _ => {}
            };
        }
    }

    shortest_path(init_pos.unwrap(), grid)
        .expect("Failed to find a path to the exit!")
}

fn puzzle_two(grid: &Grid) -> usize {
    // determine all 0 height starting points
    let init_poss: Vec<Pos> = grid.iter().enumerate().fold(Vec::new(), |mut poss, (idx, row)| {
        poss.extend(row.iter().enumerate().fold(Vec::new(), |mut poss, (jdx, sqr)| {
            if sqr.height() == 0 {
                poss.push((idx, jdx));
            }
            poss
        }));
        poss
    });

    init_poss.iter()
        .map(|init_pos| shortest_path(*init_pos, grid))
        .filter(|min_dist| min_dist.is_some())
        .map(|min_dist| min_dist.unwrap())
        .min().unwrap()
}

fn main() {
    let grid = read_input();

    println!("Puzzle 1: {}", puzzle_one(&grid));
    println!("Puzzle 2: {}", puzzle_two(&grid));
}