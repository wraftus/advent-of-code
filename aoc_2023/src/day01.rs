use std::fs;

fn read_input() -> Vec<String>{
    let file_contents: String =
        fs::read_to_string("input/day01.txt")
        .expect("Failed to read input file!");

    file_contents
        .lines()
        .map(|line: &str| line.to_string())
        .collect::<Vec<String>>()
}

fn puzzle_one(calib_lines: &Vec<String>) -> usize {
    calib_lines.iter().fold(0, |acc, line| {
        let mut first_digit: Option<usize> = None;
        for char in line.chars() {
            if char.is_numeric() {
                first_digit = Some(char.to_digit(10).unwrap() as usize);
                break;
            }
        }
        assert!(first_digit != None, "Failed to find the first digit!");

        let mut second_digit: Option<usize> = None;
        for char in line.chars().rev() {
            if char.is_numeric() {
                second_digit =  Some(char.to_digit(10).unwrap() as usize);
                break;
            }
        }
        assert!(second_digit != None, "Failed to find the second digit!");

        return acc + (first_digit.unwrap()*10 + second_digit.unwrap());
    })
}

const DIGIT_STRINGS : [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

fn puzzle_two(calib_lines: &Vec<String>) -> usize {
    calib_lines.iter().fold(0, |acc, line| {
        let mut first_digit: Option<usize> = None;
        for curr_idx in 0..line.len() {
            if line.chars().nth(curr_idx).unwrap().is_numeric() {
                let digit_char: char = line.chars().nth(curr_idx).unwrap();
                first_digit = Some(digit_char.to_digit(10).unwrap() as usize);
            }
            for (digit, digit_string) in DIGIT_STRINGS.iter().enumerate() {
                if line[..curr_idx].contains(digit_string) {
                    first_digit = Some(digit);
                }
            }
            if first_digit != None { break }
        }
        assert!(first_digit != None, "Failed to find the first digit!");

        let mut second_digit: Option<usize> = None;
        for curr_idx in (0..line.len()).rev() {
            if line.chars().nth(curr_idx).unwrap().is_numeric() {
                let digit_char: char = line.chars().nth(curr_idx).unwrap();
                second_digit = Some(digit_char.to_digit(10).unwrap() as usize);
            }
            for (digit, digit_string) in DIGIT_STRINGS.iter().enumerate() {
                if line[curr_idx..].contains(digit_string) {
                    second_digit = Some(digit);
                }
            }
            if second_digit != None { break }
        }
        assert!(second_digit != None, "Failed to find the second digit!");

        return acc + (first_digit.unwrap()*10 + second_digit.unwrap());
    })
}

fn main() {
    let calib_lines = read_input();

    println!("Puzzle 1: {}", puzzle_one(&calib_lines));
    println!("Puzzle 2: {}", puzzle_two(&calib_lines));
}
