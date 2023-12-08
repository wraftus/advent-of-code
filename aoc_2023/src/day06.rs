use std::fs;
use std::cmp::{min, max};

struct Race {
    time: usize,
    dist: usize
}

fn read_input() -> Vec<Race> {
    let file_contents =
        fs::read_to_string("input/day06.txt")
        .expect("Failed to read input file!");
    let mut file_lines = file_contents.lines();

    let time_data_strs: Vec<&str> =
        file_lines.next().expect("Missing times line from input!")
        .split_whitespace().collect();
    assert!(time_data_strs[0] == "Time:", "Malformed times line prefix!");
    let times: Vec<usize> = time_data_strs[1..].iter()
        .map(|time_str| time_str.parse().expect("Failed to parse time!")).collect();

    let record_data_strs: Vec<&str> =
        file_lines.next().expect("Missing records line from input!")
        .split_whitespace().collect();
    assert!(record_data_strs[0] == "Distance:", "Malformed distnaces line prefix!");
    let dists: Vec<usize> = record_data_strs[1..].iter()
        .map(|dist_str| dist_str.parse().expect("Failed to parse distance!")).collect();

    times.iter().zip(&dists).map(|(&time, &dist)| Race{ time, dist }).collect()
}

fn count_winnable(race: &Race) -> usize {
    // we want (T - t)*t > D
    // => -t**2 + T*t - D > 0
    // => a = -1, b = T, c = -D
    let discriminant = (race.time*race.time - 4*race.dist) as f64;
    assert!(discriminant.is_sign_positive(), "The game is not winnable!");

    let left_sol: f64  = ((race.time as f64) - discriminant.sqrt())/2f64;
    let right_sol: f64 = ((race.time as f64) + discriminant.sqrt())/2f64;

    let left_time  = max(0, (left_sol + 1f64).floor() as usize);
    let right_time = min(race.time, (right_sol - 1f64).ceil() as usize);

    return right_time - left_time + 1;
}

fn puzzle_one(races: &Vec<Race>) -> usize {
    races.iter().map(count_winnable).product()
}

fn concatenate_numbers(nums: &Vec<usize>) -> usize {
    nums.iter().fold(String::new(), |mut curr_str, num| {
        curr_str.push_str(num.to_string().as_str());
        return curr_str;
    }).parse().unwrap()
}

fn puzzle_two(races: &Vec<Race>) -> usize {
    let new_time = concatenate_numbers(&races.iter().map(|race| race.time).collect());
    let new_dist = concatenate_numbers(&races.iter().map(|race| race.dist).collect());
    count_winnable(&Race { time: new_time, dist: new_dist })
}

fn main() {
    let races = read_input();

    println!("Puzze 1: {}", puzzle_one(&races));
    println!("Puzze 2: {}", puzzle_two(&races));
}
