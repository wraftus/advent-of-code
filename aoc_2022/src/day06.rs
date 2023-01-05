use std::fs;
use std::collections::HashSet;

fn read_input() -> String {
    fs::read_to_string("input/day06.txt")
        .expect("Failed to read input file!")
}

fn unique_window_idx(sequence: &String, window_size: usize) -> usize{
    let sequence_chars: Vec<char> = sequence.chars().collect();
    for (window_idx, seq_window) in sequence_chars.windows(window_size).enumerate() {
        let seq_set: HashSet<&char> = HashSet::from_iter(seq_window);
        if seq_set.len() == window_size {
            return window_idx + window_size
        }
    }
    unreachable!("Got an empty sequence!");
}

fn puzzle_one(sequence: &String) -> usize {
    unique_window_idx(sequence, 4)
}

fn puzzle_two(sequence: &String) -> usize {
    unique_window_idx(sequence, 14)
}

fn main() {
    let sequence: String = read_input();

    println!("Puzzle 1: {}", puzzle_one(&sequence));
    println!("Puzzle 2: {}", puzzle_two(&sequence));
}