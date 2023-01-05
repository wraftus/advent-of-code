use std::fs;

enum Command {
    FRWD,
    UP,
    DOWN
}

fn parse_line(line: &str) -> (Command, usize) {
    let line_split = line.split(' ').collect::<Vec<&str>>();
    assert!(line_split.len() == 2, "Got a weird line to parse!");

    let direction: Command = match line_split[0] {
        "forward"  => Command::FRWD,
        "up"       => Command::UP,
        "down"     => Command::DOWN,
        _ => unreachable!("Unrecognized direction!")
    };

    let steps: usize = 
        line_split[1].parse::<usize>()
        .expect("Failed to parse number of steps!");

    (direction, steps)
}

fn read_input() -> Vec<(Command, usize)> {
    let file_contents = 
        fs::read_to_string("input/day02.txt")
        .expect("Failed to read input file!");

    file_contents.lines()
        .map(parse_line)
        .collect::<Vec<(Command, usize)>>()
}

fn execute_vanilla_command(command : &(Command, usize), curr_x: usize, curr_y: usize) -> (usize, usize) {
    match command.0 {
        Command::FRWD => (curr_x + command.1, curr_y),
        Command::UP   => (curr_x, curr_y - command.1),
        Command::DOWN => (curr_x, curr_y + command.1),
    }
}

fn puzzle_one(commands: &Vec<(Command, usize)>) -> usize {
    let mut curr_x: usize = 0;
    let mut curr_y: usize = 0;

    for command in commands {
        (curr_x, curr_y) = execute_vanilla_command(&command, curr_x, curr_y);
    }
    curr_x*curr_y
}

fn execute_heading_command(command : &(Command, usize), curr_x: usize, curr_y: usize, curr_h: usize) -> (usize, usize, usize) {
    match command.0 {
        Command::FRWD => (curr_x + command.1, curr_y + curr_h*command.1, curr_h),
        Command::UP   => (curr_x, curr_y, curr_h - command.1),
        Command::DOWN => (curr_x, curr_y, curr_h + command.1),
    }
}

fn puzzle_two(commands: &Vec<(Command, usize)>) -> usize {
    let mut curr_x: usize = 0;
    let mut curr_y: usize = 0;
    let mut curr_h: usize = 0;

    for command in commands {
        (curr_x, curr_y, curr_h) = execute_heading_command(command, curr_x, curr_y, curr_h);
    }
    curr_x*curr_y
}

fn main() {
    let commands: Vec<(Command, usize)> = read_input();

    println!("Puzzle 1: {}", puzzle_one(&commands));
    println!("Puzzle 2: {}", puzzle_two(&commands));
}