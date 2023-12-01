use std::fs;
use std::str::Lines;
use std::collections::HashSet;

const BOARD_SIZE: usize = 5;
struct Board {
    numbers: [[usize; BOARD_SIZE]; BOARD_SIZE],
    marked:  [[bool;  BOARD_SIZE]; BOARD_SIZE]
}
impl Board {
    fn reset_board(&mut self) {
        self.marked = [[false; BOARD_SIZE]; BOARD_SIZE];
    }

    fn mark_draw(&mut self, draw: usize) {
        for row_idx in 0..BOARD_SIZE {
            for col_idx in 0..BOARD_SIZE {
                if self.numbers[row_idx][col_idx] == draw {
                    self.marked[row_idx][col_idx] = true;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        for row_idx in 0..BOARD_SIZE {
            let mut res = true;
            for col_idx in 0..BOARD_SIZE {
                if !self.marked[row_idx][col_idx] { res = false; break; }
            }
            if res { return true; }
        }

        for col_idx in 0..BOARD_SIZE {
            let mut res = true;
            for row_idx in 0..BOARD_SIZE {
                if !self.marked[row_idx][col_idx] { res = false; break; }
            }
            if res { return true; }
        }

        return false;
    }

    fn compute_score(&self) -> usize {
        let mut score: usize = 0;
        for row_idx in 0..BOARD_SIZE {
            for col_idx in 0..BOARD_SIZE {
                if !self.marked[row_idx][col_idx] {
                    score += self.numbers[row_idx][col_idx];
                }
            }
        }
        return score;
    }
}

struct BingoGame {
    draws:  Vec<usize>,
    boards: Vec<Board>
}
impl BingoGame {
    fn reset_game(&mut self) {
        for board in self.boards.iter_mut() {
            board.reset_board();
        }
    }

    fn apply_draw(&mut self, draw_idx: usize) {
        for board in self.boards.iter_mut() {
            board.mark_draw(self.draws[draw_idx]);
        }
    }
}

fn parse_board(file_lines: &mut Lines) -> Board {
    let mut board_array = [[0; BOARD_SIZE]; BOARD_SIZE];
    for (row_idx, row_str) in file_lines.take(5).enumerate() {
        for (col_idx, num_str) in row_str.split_whitespace().enumerate() {
            let num = num_str.parse::<usize>().expect("Failed to parse board number!");
            board_array[row_idx][col_idx] = num;
        }
    }
    return Board { numbers: board_array, marked: [[false; BOARD_SIZE]; BOARD_SIZE] }
}

fn read_input() -> BingoGame {
    let file_contents =
        fs::read_to_string("input/day04.txt")
        .expect("Failed to read input file!");

    let mut file_lines = file_contents.lines();
    let draws: Vec<usize> = file_lines
        .next().expect("Input file was empty!")
        .split(',')
        .map(|num_str: &str| num_str.parse::<usize>().expect("Failed to parse drawn number!"))
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    while file_lines.next().is_some() {
        boards.push(parse_board(&mut file_lines));
    }

    return BingoGame { draws: draws, boards: boards }
}

fn puzzle_one(game: &mut BingoGame) -> usize {
    for draw_idx in 0..game.draws.len() {
        game.apply_draw(draw_idx);
        for board in game.boards.iter() {
            if board.has_won() {
                return board.compute_score() * game.draws[draw_idx];
            }
        }
    }
    unreachable!("No boards won!")
}

fn puzzle_two(game: &mut BingoGame) -> usize {
    let mut completed_boards: HashSet<usize> = HashSet::new();
    for draw_idx in 0..game.draws.len() {
        game.apply_draw(draw_idx);
        for (board_idx, board) in game.boards.iter().enumerate() {
            if completed_boards.contains(&board_idx) { continue; }
            if board.has_won() {
                if (completed_boards.len() + 1) == game.boards.len() {
                    return board.compute_score() * game.draws[draw_idx];
                }
                completed_boards.insert(board_idx);
            }
        }
    }
    unreachable!("Some boards did not win!")
}

fn main() {
    let mut game = read_input();

    println!("Puzzle 1: {}", puzzle_one(&mut game));
    game.reset_game();
    println!("Puzzle 1: {}", puzzle_two(&mut game));
}
