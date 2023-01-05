use std::fs;
use regex::Regex;
use std::cmp;

type IDs = (usize, usize);

fn read_in_ids() -> Vec<(IDs, IDs)> {
    let file_contents: String =
        fs::read_to_string("input/day04.txt")
        .expect("Failed to read input file!");

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    file_contents.as_str()
        .split("\n")
        .map(|line| {
            let captures = re.captures(line).expect(format!("Failed to regex line {}", line).as_str());
            assert!(captures.len() == 5, "Line {} has {} captures!", line, captures.len());

            let ids1: IDs = (
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
            );
            let ids2: IDs = (
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<usize>().unwrap()
            );
            (ids1, ids2)
        }).collect::<Vec<(IDs, IDs)>>()
    }

fn ranges_overlap(ids1: &IDs, ids2: &IDs) -> bool {
    (ids1.0 <= ids2.0 && ids1.1 >= ids2.1) 
                      ||
    (ids2.0 <= ids1.0 && ids2.1 >= ids1.1)
}

fn puzzle_one(id_pairs : &Vec<(IDs, IDs)>) -> usize{
    id_pairs.iter().map(
        |id_pair: &(IDs, IDs)| ranges_overlap(&id_pair.0, &id_pair.1) as usize
    ).sum()
}

fn num_overlapping(ids1: &IDs, ids2: &IDs) -> bool {
    !((ids1.1 < ids2.0) || (ids2.1 < ids1.0))

}

fn puzzle_two(id_pairs : &Vec<(IDs, IDs)>) -> usize {
    id_pairs.iter().map(
        |id_pair: &(IDs, IDs)| num_overlapping(&id_pair.0, &id_pair.1) as usize
    ).sum()
}

fn main() {
    let id_pairs: Vec<(IDs, IDs)> = read_in_ids();

    println!("Puzzle 1: {}", puzzle_one(&id_pairs));
    println!("Puzzle 1: {}", puzzle_two(&id_pairs));
}