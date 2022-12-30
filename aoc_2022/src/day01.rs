use std::fs;

// read in the calories for each elf
fn read_in_cals() -> Vec<Vec<usize>>{
    // extract the contents of the file as a string
    let file_contents: String = 
        fs::read_to_string("input/day01.txt")
        .expect("Failed to read input file!");

    // seperate the string into vecs of calories for each elf
    file_contents.as_str()
        .split("\n\n").map(
            |elve_cals: &str| elve_cals.split("\n").map(
                |cal: &str| cal.parse::<usize>().expect("Failed to parse a calorie!")
            ).collect::<Vec<usize>>()
        ).collect::<Vec<Vec<usize>>>()
}

fn puzzle_one(elves_cals: &Vec<Vec<usize>>) -> usize {
    // compute the elf with the max total calories
    elves_cals.iter().map(
        |cals: &Vec<usize>| cals.iter().sum()
    ).max().unwrap()
}

fn puzzle_two(evles_cals: &Vec<Vec<usize>>) -> usize {
    // compute the total calories for all elves
    let mut all_sums: Vec<usize> =
        evles_cals.iter().map(
            |cals: &Vec<usize>| cals.iter().sum()
        ).collect::<Vec<usize>>();

    // sort them and return the largest three
    all_sums.sort();
    all_sums.iter().rev().take(3).sum()
}

fn main(){
    let elves_cals: Vec<Vec<usize>> = read_in_cals();

    println!("Puzzle 1: {}", puzzle_one(&elves_cals));
    println!("Puzzle 1: {}", puzzle_two(&elves_cals));
}
