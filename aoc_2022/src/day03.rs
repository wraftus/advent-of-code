use std::fs;
use std::collections::HashSet;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
enum TripleCount {
    None,
    One,
    Two,
    Three
}
impl TripleCount {
    fn increment(&self) -> TripleCount {
        match self {
            TripleCount::None  => TripleCount::One,
            TripleCount::One   => TripleCount::Two,
            TripleCount::Two   => TripleCount::Three,
            TripleCount::Three => unreachable!("An item shows up more than three times!")
        }
    }
}

fn read_in_rucksacks() -> Vec<Vec<char>> {
    let file_contents: String = 
        fs::read_to_string("input/day03.txt")
        .expect("Failed to read input file!");

    file_contents.as_str()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

// item's char to it's priority
fn char_priority(c: &char) -> usize {
    assert!(c.is_alphabetic(), "Char is not alphabetic!");
    if c.is_lowercase() {
        (*c as usize) - 96usize // a is ascii #97, i.e. "a" - 97 + 1 = "a" - 96
    } else {
        (*c as usize) - 38usize // A is ascii #65, i.e. "A" - 65 + 27 = "A" - 38
    }
}

// computes the priority of the overlapping item
fn compartment_overlap(rucksack: &Vec<char>) -> usize {
    let num_chars: usize = rucksack.len();
    assert!(num_chars % 2 == 0, "Found an odd number of items in the rucksack!");

    // create sets for items in each compartment
    let (compartment1, compartment2): (&[char], &[char]) = rucksack.split_at(num_chars/2);
    let compartment1_set: HashSet<&char> = HashSet::from_iter(compartment1.iter());
    let compartment2_set: HashSet<&char> = HashSet::from_iter(compartment2.iter());

    // determine which items are in the first compartment
    let mut in_compartment1: [bool; 52] = [false; 52usize];
    for c in compartment1_set {
        in_compartment1[char_priority(c) - 1] = true;
    }

    // look for the overlap in the second compartment
    for c in compartment2_set {
        let priority: usize = char_priority(c);
        if in_compartment1[priority - 1]{ 
            return priority
         }
    }
    unreachable!("Failed to find an overlap in the rucksack!")
}

// sum up the priorities of each overlap
fn puzzle_one(rucksacks: &Vec<Vec<char>>) -> usize {
    rucksacks.iter().map(compartment_overlap).sum()
}

fn group_overlap(rucksacks: &[Vec<char>]) -> usize {
    let rucksack_sets: Vec<HashSet<&char>> = 
        rucksacks.iter().map(
            |rucksack| HashSet::from_iter(rucksack.iter())
        ).collect();

    let mut items_in_rucksacks: [TripleCount; 52] = [TripleCount::None; 52];
    for rucksack_set in rucksack_sets {
        for c in rucksack_set {
            let priority: usize = char_priority(c);
            items_in_rucksacks[priority - 1] =  items_in_rucksacks[priority - 1].increment();
        }
    }

    for priority in 1..53 {
        if items_in_rucksacks[priority-1] == TripleCount::Three {
            return priority
        }
    }
    unreachable!("Failed to find an item showing up in all three!")
}

fn puzzle_two(rucksacks: &Vec<Vec<char>>) -> usize {
    assert!(rucksacks.len() % 3 == 0, "Number of rucksacks is not a multiple of 3!");
    rucksacks.chunks(3).map(group_overlap).sum()
}

fn main() {
    let rucksacks: Vec<Vec<char>> = read_in_rucksacks();

    println!("Puzzle 1: {}", puzzle_one(&rucksacks));
    println!("Puzzle 2: {}", puzzle_two(&rucksacks));
}