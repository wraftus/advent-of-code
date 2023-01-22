use std::fs;
use std::borrow::BorrowMut;

use regex::Regex;

struct StackInstr {
    num  : usize,
    from : usize,
    to   : usize
}
impl StackInstr {
    fn exectue_single_move(&self, stacks: &mut Vec<Vec<char>>) {
        assert!(stacks.len() >= self.from && stacks.len() >= self.to, "Can't execute instruction, not enough stacks!");
        assert!(stacks[self.from - 1].len() >= self.num, "Can't execute instruction, stack too small!");
        for _ in 0..self.num {
            let to_push: char = stacks.get_mut(self.from - 1).unwrap().pop().unwrap();
            stacks.get_mut(self.to - 1).unwrap().push(to_push);
        }
    }

    fn execute_all_move(&self, stacks: &mut Vec<Vec<char>>) {
        assert!(stacks.len() >= self.from && stacks.len() >= self.to, "Can't execute instruction, not enough stacks!");
        assert!(stacks[self.from - 1].len() >= self.num, "Can't execute instruction, stack too small!");

        let mut to_push: Vec<char> = Vec::new();
        for _ in 0..self.num {
            let curr_char: char = stacks.get_mut(self.from - 1).unwrap().pop().unwrap();
            to_push.push(curr_char);
        }
        while to_push.len() > 0 {
            stacks.get_mut(self.to - 1).unwrap().push(to_push.pop().unwrap());
        }
    }
}

fn parse_stacks(stack_lines: &Vec<&str>) -> Vec<Vec<char>> {
    let num_stack_lines = stack_lines.len();
    assert!(num_stack_lines > 0, "Stack lines was empty!");
    let num_stacks : usize = stack_lines[num_stack_lines-1]
        .trim().split("   ")
        .map(|num_str|
            num_str.parse::<usize>()
                   .expect(format!("Failed to parse stack number [{}]!", num_str).as_str())
        ).last().unwrap();
    assert!(num_stacks > 0usize, "No stacks found!");

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    for stack_line in stack_lines[0..(num_stack_lines-1)].iter().rev() {
        assert!(stack_line.len() > (num_stacks-1)*4 + 1, "stack line has is not long enough!");
        for stack_idx in 0..num_stacks {
            let stack_char: char = stack_line.chars().nth(stack_idx*4 + 1).unwrap();
            if stack_char.is_whitespace() { continue; }

            assert!(stack_char.is_alphabetic(), "The item to add to this stack is not alphabetic");
            stacks[stack_idx].push(stack_char);
        }
    }
    return stacks;
}

fn parse_instrs(instr_lines: &Vec<&str>) -> Vec<StackInstr> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    instr_lines.iter().map(|line| {
            let captures = re.captures(line).expect(format!("Failed to regex line {}", line).as_str());
            assert!(captures.len() == 4, "Line {} has {} captures", line, captures.len());

            let num  = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to   = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            StackInstr { num: num, from: from, to: to }
        }).collect::<Vec<StackInstr>>()
}

fn parse_input() -> (Vec<Vec<char>>, Vec<StackInstr>) {
    let file_contents = 
        fs::read_to_string("input/day05.txt")
        .expect("Failed to read input file!");

    let mut file_lines = file_contents.as_str().lines();
    let stack_lines = file_lines.by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let instr_lines = file_lines.by_ref().collect::<Vec<&str>>();

    (parse_stacks(&stack_lines), parse_instrs(&instr_lines))
}

fn puzzle_one(stacks: &Vec<Vec<char>>, instrs: &Vec<StackInstr>) -> String {
    let mut stacks = stacks.iter().map(|stack| stack.to_vec()).collect::<Vec<Vec<char>>>();
    for instr in instrs {
        instr.exectue_single_move(stacks.borrow_mut());
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>()
}

fn puzzle_two(stacks: &Vec<Vec<char>>, instrs: &Vec<StackInstr>) -> String {
    let mut stacks = stacks.iter().map(|stack| stack.to_vec()).collect::<Vec<Vec<char>>>();
    for instr in instrs {
        instr.execute_all_move(stacks.borrow_mut());
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>()
}

fn main() {
    let (stacks, instrs) =  parse_input();

    println!("Puzzle 1: {}", puzzle_one(&stacks, &instrs));
    println!("Puzzle 2: {}", puzzle_two(&stacks, &instrs));
}