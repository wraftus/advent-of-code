use std::fs;
use std::cmp::min;
use std::collections::HashSet;

use regex::Regex;

struct ScratchCard {
    drawn:   HashSet<usize>,
    winning: HashSet<usize>
}
impl ScratchCard {
    fn num_matches(&self) -> usize {
        self.drawn.intersection(&self.winning).count()
    }
    fn score_from_matches(matches: usize) -> usize {
        if matches == 0 { return 0; }
        return (2usize).pow(((matches as isize) - 1) as u32)
    }
    fn score(&self) -> usize {
        let num_overlap = self.num_matches();
        ScratchCard::score_from_matches(num_overlap)
    }
}

fn parse_numbers(numbers_str: &str) -> HashSet<usize> {
    HashSet::from_iter(
        numbers_str.split_whitespace().map(|num_str| num_str.parse().unwrap())
    )
}

fn read_input() -> Vec<ScratchCard> {
    let file_contents =
    fs::read_to_string("input/day04.txt")
    .expect("Failed to read input file!");

    let line_re = Regex::new(r"Card\s+\d+:(.+)\|(.+)").unwrap();
    file_contents.lines().map(|line_str: &str| {
        let captures = line_re.captures(line_str).unwrap();
        assert!(captures.len() == 2+1, "Malformed number of line captures!");
        ScratchCard {
            drawn:   parse_numbers(captures.get(1).unwrap().as_str()),
            winning: parse_numbers(captures.get(2).unwrap().as_str())
        }
    }).collect()
}

fn puzzle_one(scratch_cards: &Vec<ScratchCard>) -> usize {
    scratch_cards.iter().map(|card: &ScratchCard| card.score()).sum()
}

fn puzzle_two(scratch_cards: &Vec<ScratchCard>) -> usize {
    let mut card_counts: Vec<usize> = vec![1; scratch_cards.len()];
    for (card_idx, card) in scratch_cards.iter().enumerate() {
        let matches = card.num_matches();
        for other_idx in (card_idx + 1)..min(card_idx + matches + 1, scratch_cards.len()) {
            card_counts[other_idx] += card_counts[card_idx];
        }
    }
    return card_counts.iter().sum();
}

fn main() {
    let scratch_cards = read_input();

    println!("Puzzle 1: {}", puzzle_one(&scratch_cards));
    println!("Puzzle 2: {}", puzzle_two(&scratch_cards));
}
