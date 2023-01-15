use std::fs;

enum Instr {
    NoOp,
    Add(isize)
}
impl Instr {
    fn parse(line: &str) -> Instr {
        match line {
            "noop" => Instr::NoOp,
            _      => {
                let split_line = line.split(" ").collect::<Vec<&str>>();
                assert!(split_line.len() == 2, "Poorly formed instruction!");
                assert!(split_line[0] == "addx", "Unrecognized command!");

                let num = split_line[1].parse::<isize>().expect("Failed to parse number to add!");
                Instr::Add(num)
            }
        }
    }
}

#[derive(PartialEq)]
enum State {
    Start,
    ExecNoOp,
    WaitAdd(isize),
    ExecAdd(isize),
    Done
}
impl State {
    fn from_instr(instr: Option<&Instr>) -> State {
        match instr {
            None => State::Done,
            Some(instr) => {
                match instr {
                    Instr::NoOp    => State::ExecNoOp,
                    Instr::Add(x)  => State::WaitAdd(*x)
                }
            }
        }
    }
}

fn read_input() -> Vec<Instr> {
    let file_contents = 
        fs::read_to_string("input/day10.txt")
        .expect("Failed to read input file!");

    file_contents.lines()
        .map(Instr::parse)
        .collect::<Vec<Instr>>()
}

fn puzzle_one(instrs: &Vec<Instr>) -> isize {
    let mut instrs = instrs.iter();
    let mut state = State::Start;
    let mut cycle: isize = 0;
    let mut reg_val: isize = 1;
    let mut strength: isize = 0;

    while state != State::Done {
        // see if we check the strength this cycle
        if ((cycle - 20) % 40) == 0 {
            strength += cycle * reg_val;
        }

        // execute operation and update the state
        match state {
            State::Start      => state = State::from_instr(instrs.next()),
            State::ExecNoOp   => state = State::from_instr(instrs.next()),
            State::WaitAdd(x) => state = State::ExecAdd(x),
            State::ExecAdd(x) => {
                reg_val += x;
                state = State::from_instr(instrs.next());
            },
            State::Done => unreachable!("Shoule not by looping in state done!")
        }
        cycle += 1;
    }
    strength
}

fn puzzle_two(instrs: &Vec<Instr>) -> String {
    let mut instrs = instrs.iter();
    let mut state = State::Start;
    let mut cycle: isize = 0;
    let mut reg_val: isize = 1;
    let mut pixels: Vec<char> = Vec::new();

    while state != State::Done {
        // execute operation and update the state
        match state {
            State::Start      => state = State::from_instr(instrs.next()),
            State::ExecNoOp   => state = State::from_instr(instrs.next()),
            State::WaitAdd(x) => state = State::ExecAdd(x),
            State::ExecAdd(x) => {
                reg_val += x;
                state = State::from_instr(instrs.next());
            },
            State::Done => unreachable!("Shoule not by looping in state done!")
        }

        // add this cycles pixel
        if ((cycle % 40) - reg_val).abs() <= 1 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }

        // update cycle and add new line if necessary
        cycle += 1;
        if (cycle % 40) == 0 {
            pixels.push('\n');
        }
    }
    pixels.iter().collect::<String>()
}

fn main() {
    let instrs: Vec<Instr> = read_input();

    println!("Puzzle 1: {}", puzzle_one(&instrs));
    println!("Puzzle 2:\n{}", puzzle_two(&instrs));
}