use std::fs;
use std::collections::HashSet;

type Grid = Vec<Vec<usize>>;
fn read_input() -> Grid {
    let file_contents =
        fs::read_to_string("input/day08.txt")
        .expect("Failed to read input file!");

    file_contents.lines().map(|line| {
        line.chars().map(|c| {
            c.to_digit(10).expect("Failed to parse height!") as usize
        }).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>()
}

fn visable_idxs(heights: &Vec<usize>) -> Vec<usize>{
    let mut visable_idxs: Vec<usize> = Vec::new();
    visable_idxs.extend(vec![0, heights.len()-1]);

    let mut max_height = heights.first().unwrap();
    for idx in 1..heights.len() {
        let height = &heights[idx];
        if height > max_height { 
            visable_idxs.push(idx);
            max_height = height;
        }
    }

    max_height = heights.last().unwrap();
    for idx in (0..(heights.len() - 1)).rev() {
        let height = &heights[idx];
        if height > max_height {
            visable_idxs.push(idx);
            max_height = height;
        }
    }

    visable_idxs
}

fn puzzle_one(height_grid: &Grid) -> usize {
    let mut visable_poss = HashSet::new();
    
    for row_idx in 1..(height_grid.len() - 1) {
        let row = &height_grid[row_idx];
        visable_poss.extend(
            visable_idxs(row)
            .iter().map(|col_idx| format!("{:03}{:03}", row_idx, col_idx))
        );
    }

    for col_idx in 1..(height_grid.first().unwrap().len() - 1) {
        let col = height_grid.iter().map(|row| row[col_idx]).collect::<Vec<usize>>();
        visable_poss.extend(
            visable_idxs(&col)
            .iter().map(|row_idx| format!("{:03}{:03}", row_idx, col_idx))
        );
    }

    visable_poss.len() + 4
}

fn sight_line_score(sight_line: &Vec<usize>, idx: usize) -> usize {
    let base_height = &sight_line[idx];

    let mut bfore_visable = 0;
    for idx in  (0..idx).rev() {
        bfore_visable += 1;
        if &sight_line[idx] >= base_height { break; }
    }

    let mut after_visable = 0;
    for idx in (idx + 1)..sight_line.len() {
        after_visable += 1;
        if &sight_line[idx] >= base_height { break; }
    }

     bfore_visable * after_visable
}

fn puzzle_two(height_grid: &Grid) -> usize {
    let mut max_score = 0;

    for col_idx in 1..(height_grid.first().unwrap().len() - 1) {
        let col = height_grid.iter().map(
            |row| row[col_idx]
        ).collect::<Vec<usize>>();

        for row_idx in 1..(height_grid.len() -1) {
            let col_score = sight_line_score(&col, row_idx);
            let row_score = sight_line_score(&height_grid[row_idx], col_idx);
            let score = col_score * row_score;

            if score >= max_score { max_score = score }
        }
    }

    max_score
}

fn main() {
    let height_grid: Grid = read_input();

    println!("Puzzle 1: {}", puzzle_one(&height_grid));
    println!("Puzzle 2: {}", puzzle_two(&height_grid));
}