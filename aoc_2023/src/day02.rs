use std::fs;
use regex::Regex;

struct Draw {
    num_red:   usize,
    num_green: usize,
    num_blue:  usize
}

const MAX_RED:   usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE:  usize = 14;
struct Game {
    draws: Vec<Draw>
}
impl Game {
    fn verify(&self) -> bool {
        for draw in self.draws.iter() {
            if draw.num_red   > MAX_RED   { return false; }
            if draw.num_green > MAX_GREEN { return false; }
            if draw.num_blue  > MAX_BLUE  { return false; }
        }
        return true;
    }

    fn power(&self) -> usize {
        let mut min_red   = self.draws[0].num_red;
        let mut min_green = self.draws[0].num_green;
        let mut min_blue  = self.draws[0].num_blue;
        for draw in self.draws[1..].iter() {
            if draw.num_red   > min_red   { min_red   = draw.num_red; }
            if draw.num_green > min_green { min_green = draw.num_green; }
            if draw.num_blue  > min_blue  { min_blue  = draw.num_blue; }
        }
        return min_red*min_green*min_blue;
    }
}

fn parse_draw(draw_string: &str) -> Draw {
    let mut color_draws: [usize; 3] = [0; 3];
    for color_draw_str in draw_string.split(", ") {
        let color_draw_split: Vec<&str> = color_draw_str.split(" ").collect();
        assert!(color_draw_split.len() == 2, "Malformed color draw!");

        let num_drawn: usize = color_draw_split[0].parse()
            .expect("Failed to parse num drawn!");
        match color_draw_split[1] {
            "red"   => color_draws[0] = num_drawn,
            "green" => color_draws[1] = num_drawn,
            "blue"  => color_draws[2] = num_drawn,
            _       => unreachable!("Unrecognized color!")
        }
    }

    return Draw {
        num_red:   color_draws[0],
        num_green: color_draws[1],
        num_blue:  color_draws[2]
    }
}

fn read_input() -> Vec<Game> {
    let file_contents: String =
        fs::read_to_string("input/day02.txt")
        .expect("Failed to read input file1");

    let game_regex = Regex::new(r"Game \d+: (.*)$").unwrap();
    file_contents.lines()
        .map(|game_str: &str| {
            let captures = game_regex
                .captures(game_str)
                .expect("Failed to regex game line!");
            assert!(captures.len() == 1+1, "Line has more than one capture!");
            let game_str = captures.get(1).unwrap().as_str();

            Game { draws: game_str.split("; ").map(parse_draw).collect() }
        }).collect()
}

fn puzzle_one(games: &Vec<Game>) -> usize {
    let mut counter: usize = 0;
    for (game_idx, game) in games.iter().enumerate() {
        if game.verify() {
            counter += game_idx + 1;
        }
    }
    return counter;
}

fn puzzle_two(games: &Vec<Game>) -> usize {
    games.iter().map(Game::power).sum()
}

fn main() {
    let games = read_input();

    println!("Puzzle 1: {}", puzzle_one(&games));
    println!("Puzzle 2: {}", puzzle_two(&games));
}
