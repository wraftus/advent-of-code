use std::fs;
use regex::Regex;

struct Number {
    value: usize,
    start: usize,
    width: usize
}

struct Symbol {
    row:     usize,
    column:  usize,
    is_gear: bool
}

fn read_input() -> (Vec<Vec<Number>>, Vec<Symbol>) {
    let file_contents =
        fs::read_to_string("input/day03.txt")
        .expect("Failed to read input file!");


    let number_re = Regex::new(r"(\d+)").unwrap();
    let mut numbers: Vec<Vec<Number>> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (line_num, line_str) in file_contents.lines().enumerate() {
        let mut line_numbers: Vec<Number> = Vec::new();
        for number_match in number_re.find_iter(line_str) {
            let num_str = number_match.as_str();
            line_numbers.push(Number {
                value: num_str.parse().unwrap(),
                start: number_match.start(),
                width: num_str.len()
            });
        }
        numbers.push(line_numbers);

        for (char_num, char) in line_str.chars().enumerate() {
            if char.is_numeric() { continue; }
            if char == '.'       { continue; }
            symbols.push(Symbol { row: line_num, column: char_num, is_gear: char == '*' });
        }
    }

    (numbers, symbols)
}

fn symbol_adjacent_number(number: &Number, symbol: &Symbol) -> bool {
    let left_bound  = if number.start == 0 { 0 } else { number.start - 1 };
    let right_bound = number.start + number.width;
    return (left_bound <= symbol.column) && (symbol.column <= right_bound);
}

fn puzzle_one(numbers: &Vec<Vec<Number>>, symbols: &Vec<Symbol>) -> usize {
    let mut summer: usize = 0;
    for symbol in symbols.iter() {
        for number in numbers[symbol.row].iter() {
            if (number.start + number.width) == symbol.column {
                summer += number.value;
            }
            else if (symbol.column + 1) == number.start {
                summer += number.value;
            }
        }
        if symbol.row > 0 {
            let above_row = (symbol.row as isize) - 1;
            for number in numbers[above_row as usize].iter() {
                if symbol_adjacent_number(number, symbol) {
                    summer += number.value;
                }

            }
        }
        if symbol.row + 1 < numbers.len() {
            let below_row = symbol.row + 1;
            for number in numbers[below_row].iter() {
                if symbol_adjacent_number(number, symbol) {
                    summer += number.value;
                }
            }
        }
    }
    return summer;
}

fn puzzle_two(numbers: &Vec<Vec<Number>>, symbols: &Vec<Symbol>) -> usize {
    let mut summer: usize = 0;
    for symbol in symbols.iter() {
        if !symbol.is_gear { continue; }

        let mut symbol_numbers: Vec<&Number> = Vec::new();
        for number in numbers[symbol.row].iter() {
            if (number.start + number.width) == symbol.column {
                symbol_numbers.push(number);
            }
            else if (symbol.column + 1) == number.start {
                symbol_numbers.push(number);
            }
        }
        if symbol.row > 0 {
            let above_row = (symbol.row as isize) - 1;
            for number in numbers[above_row as usize].iter() {
                if symbol_adjacent_number(number, symbol) {
                    symbol_numbers.push(number);
                }
            }
        }
        if symbol.row + 1 < numbers.len() {
            let below_row = symbol.row + 1;
            for number in numbers[below_row].iter() {
                if symbol_adjacent_number(number, symbol) {
                    symbol_numbers.push(number);
                }
            }
        }

        if symbol_numbers.len() == 2 {
            summer += symbol_numbers.iter().fold(1, |acc, number| acc*number.value);
        }
    }
    return summer;
}

fn main() {
    let (numbers, symbols) = read_input();

    println!("Puzzle 1: {}", puzzle_one(&numbers, &symbols));
    println!("Puzzle 2: {}", puzzle_two(&numbers, &symbols));
}
