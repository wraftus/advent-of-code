use std::fs;
use std::collections::HashSet;
use std::cmp::{min, max};

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    fn parse(dir_str: &str) -> Option<Direction> {
        match dir_str {
            "U" => Some(Direction::UP),
            "D" => Some(Direction::DOWN),
            "L" => Some(Direction::LEFT),
            "R" => Some(Direction::RIGHT),
            _ => None
        }
    }

    fn move_from(&self, cur_pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::UP    => (    cur_pos.0, cur_pos.1 + 1),
            Direction::DOWN  => (    cur_pos.0, cur_pos.1 - 1),
            Direction::LEFT  => (cur_pos.0 - 1,     cur_pos.1),
            Direction::RIGHT => (cur_pos.0 + 1,     cur_pos.1)
        }
    }
}

fn read_input() -> Vec<(Direction, isize)> {
    let file_contents =
        fs::read_to_string("input/day09.txt")
        .expect("Failed to read file contents!");

    file_contents.lines().map(|line| {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        assert!(split_line.len() == 2, "Got poorly formatted line!");

        let dir =
            Direction::parse(split_line[0])
            .expect("Failed to parse direction!");
        let dist = 
            split_line[1].parse::<isize>()
            .expect("Failed to parse distance!");
        (dir, dist)
    }).collect::<Vec<(Direction, isize)>>()
}

fn clip_move(d: isize) -> isize {
    if d > 0 {
        min(d, 1)
    } else {
        max(d, -1)
    }
}

fn follow_head(head_pos: (isize, isize), tail_pos: (isize, isize)) -> (isize, isize) {
    let disp: (isize, isize) = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);
    if max(disp.0.abs(), disp.1.abs()) <= 1 {
        return tail_pos;
    }

    let tail_del = (clip_move(disp.0), clip_move(disp.1));
    (tail_pos.0 + tail_del.0, tail_pos.1 + tail_del.1)
}

fn puzzle_one(motions: &Vec<(Direction, isize)>) -> usize {
    let mut head_pos: (isize, isize) = (0, 0);
    let mut tail_pos: (isize, isize) = (0, 0);
    let mut tail_poss: HashSet<String> = HashSet::new();

    for (dir, dist) in motions {
        for _ in 0..*dist {
            head_pos = dir.move_from(head_pos);
            tail_pos = follow_head(head_pos, tail_pos);
            tail_poss.insert(format!("{:03}{:03}", tail_pos.0, tail_pos.1));
        }
    }
    tail_poss.len()
}

const NUM_KNOTS: usize = 10;
fn puzzle_two(motions: &Vec<(Direction, isize)>) -> usize {
    let mut cur_poss: [(isize, isize); NUM_KNOTS] = [(0, 0); NUM_KNOTS];
    let mut tail_poss: HashSet<String> = HashSet::new();

    for (dir, dist) in motions {
        for _ in 0..*dist {
            cur_poss[0] = dir.move_from(cur_poss[0]);
            for knot_idx in 1..NUM_KNOTS {
                let new_pos = follow_head(cur_poss[knot_idx - 1], cur_poss[knot_idx]);
                cur_poss[knot_idx] = new_pos;
            }
            let tail_pos = cur_poss.last().unwrap();
            tail_poss.insert(format!("{:03}{:03}", tail_pos.0, tail_pos.1));
        }
    }
    tail_poss.len()
}

fn main() {
    let motions: Vec<(Direction, isize)> = read_input();

    println!("Puzzle 1: {}", puzzle_one(&motions));
    println!("Puzzle 2: {}", puzzle_two(&motions));
}