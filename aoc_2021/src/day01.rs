use std::fs;

fn read_in_input() -> Vec<usize> {
    let file_contents =
        fs::read_to_string("input/day01.txt")
        .expect("Failed to read input file!");

    file_contents.lines()
        .map(|line| line.parse::<usize>().expect("Failed to parse line!"))
        .collect::<Vec<usize>>()
}

fn num_incrementing(numbers: &Vec<usize>) -> usize {
    let mut incr_count: usize = 0;

    let mut prev_measure = numbers[0];
    for measure in &numbers[1..] {
        if measure > &prev_measure { 
            incr_count+= 1; 
        }
        prev_measure = measure.clone();
    }
    incr_count
}

fn puzzle_one(measures: &Vec<usize>) -> usize {
    num_incrementing(measures)
}

fn puzzle_two(measures: &Vec<usize>) -> usize {
    let window_sums = measures.windows(3)
        .map(|window| window.to_vec().iter().sum())
        .collect::<Vec<usize>>();

    num_incrementing(&window_sums)
}

fn main() {
    let measures: Vec<usize> = read_in_input();

    println!("Puzzle 1: {}", puzzle_one(&measures));
    println!("Puzzle 1: {}", puzzle_two(&measures))
}
