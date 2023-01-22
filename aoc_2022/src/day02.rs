use std::fs;
use std::iter::zip;

enum RPSMove {
    ROCK,
    PAPER,
    SCISSORS,
}
impl RPSMove {
    fn score(&self) -> u32 {
        match self {
            RPSMove::ROCK     => 1u32,
            RPSMove::PAPER    => 2u32,
            RPSMove::SCISSORS => 3u32
        }
    }
}

enum RPSOutcome {
    LOSE,
    DRAW,
    WIN
}
impl RPSOutcome {
    fn score(&self) -> u32 {
        match self {
            RPSOutcome::LOSE => 0u32,
            RPSOutcome::DRAW => 3u32,
            RPSOutcome::WIN  => 6u32
        }
    }
}

enum RPSStrat {
    X,
    Y,
    Z
}
impl RPSStrat {
    fn as_move(&self) -> RPSMove {
        match self {
            RPSStrat::X => RPSMove::ROCK,
            RPSStrat::Y => RPSMove::PAPER,
            RPSStrat::Z => RPSMove::SCISSORS
        }
    }

    fn as_outcome(&self) -> RPSOutcome {
        match self {
            RPSStrat::X => RPSOutcome::LOSE,
            RPSStrat::Y => RPSOutcome::DRAW,
            RPSStrat::Z => RPSOutcome::WIN
        }
    }
}

// parse an input line from the string to RPSmove
fn parse_line(line : &str) -> (RPSMove, RPSStrat) {
    let player_moves: Vec<&str> = line.split(" ").collect();
    if player_moves.len() != 2 {
        unreachable!("Invalid input line, got more than 2 items!");
    }
    
    let opp_move: RPSMove =
        match player_moves[0] {
            "A" => RPSMove::ROCK,
            "B" => RPSMove::PAPER,
            "C" => RPSMove::SCISSORS,
            _ => unreachable!("Invalid input, unrecognized player 1 move!")
        };

    let strat: RPSStrat =
        match player_moves[1] {
            "X" => RPSStrat::X,
            "Y" => RPSStrat::Y,
            "Z" => RPSStrat::Z,
            _ => unreachable!("Invalid input, unrecognized player 2 move!")
        };
    (opp_move, strat)
}

fn read_in_strats() -> (Vec<RPSMove>, Vec<RPSStrat>) {
    let file_contents: String = 
        fs::read_to_string("input/day02.txt")
        .expect("Failed to read input file!");

    file_contents.as_str()
        .lines()
        .map(parse_line)
        .unzip()
}

// determine the outcome of a match given moves of each player
fn determine_outcome(opp_move: &RPSMove, you_move: &RPSMove) -> RPSOutcome {
    match opp_move {
        RPSMove::ROCK    => 
            match you_move {
                RPSMove::ROCK     => RPSOutcome::DRAW,
                RPSMove::PAPER    => RPSOutcome::WIN,
                RPSMove::SCISSORS => RPSOutcome::LOSE
            },

        RPSMove::PAPER   => 
            match you_move {
                RPSMove::ROCK     => RPSOutcome::LOSE,
                RPSMove::PAPER    => RPSOutcome::DRAW,
                RPSMove::SCISSORS => RPSOutcome::WIN
            },

        RPSMove::SCISSORS =>
            match you_move {
                RPSMove::ROCK     => RPSOutcome::WIN,
                RPSMove::PAPER    => RPSOutcome::LOSE,
                RPSMove::SCISSORS => RPSOutcome::DRAW
            }
    }
}

fn puzzle_one(opp_moves: &Vec<RPSMove>, strats: &Vec<RPSStrat>) -> u32 {
    // turn the strategies into moves
    let you_moves: Vec<RPSMove> = 
        strats.iter().map(
            |strat: &RPSStrat| strat.as_move()
        ).collect::<Vec<RPSMove>>();

    // determine the score of each match given the moves
    zip(opp_moves, you_moves).map(
        |(opp_move, you_move): (&RPSMove, RPSMove)| determine_outcome(opp_move, &you_move).score() + you_move.score()
    ).sum::<u32>()
}

// determine the move you should make given the opp's move and the desired outcome
fn determine_move(opp_move: &RPSMove, outcome: &RPSOutcome) -> RPSMove{
    match opp_move {
        RPSMove::ROCK    => 
            match outcome {
                RPSOutcome::LOSE => RPSMove::SCISSORS,
                RPSOutcome::DRAW => RPSMove::ROCK,
                RPSOutcome::WIN  => RPSMove::PAPER
            },
        RPSMove::PAPER   => 
            match outcome {
                RPSOutcome::LOSE => RPSMove::ROCK,
                RPSOutcome::DRAW => RPSMove::PAPER,
                RPSOutcome::WIN  => RPSMove::SCISSORS
            },
        RPSMove::SCISSORS =>
            match outcome {
                RPSOutcome::LOSE => RPSMove::PAPER,
                RPSOutcome::DRAW => RPSMove::SCISSORS,
                RPSOutcome::WIN  => RPSMove::ROCK
            }
    }
}

fn puzzle_two(opp_moves: &Vec<RPSMove>, strats: &Vec<RPSStrat>) -> u32 {
    // turn the strategies into desired outcomes
    let outcomes: Vec<RPSOutcome> = 
        strats.iter().map(
            |strat: &RPSStrat| strat.as_outcome()
        ).collect::<Vec<RPSOutcome>>();
        
    // determine the score of each match given opp's move and desired outcome
    zip(opp_moves, outcomes).map(
        |(opp_move, outcome): (&RPSMove, RPSOutcome)| determine_move(opp_move, &outcome).score() + outcome.score()
    ).sum::<u32>()
}

fn main() {
    let (opp_moves, strats): (Vec<RPSMove>, Vec<RPSStrat>) = read_in_strats();

    println!("Puzzle 1: {}", puzzle_one(&opp_moves, &strats));
    println!("Puzzle 2: {}", puzzle_two(&opp_moves, &strats));
}