use std::fs;

struct Sequence {
    nums: Vec<isize>
}
impl Sequence {
    fn take_differences(&self) -> Sequence {
        let diffs: Vec<isize> = self.nums.iter()
            .zip(self.nums.iter().skip(1))
            .map(|(num_curr, num_next)| num_next - num_curr)
            .collect();
        Sequence { nums: diffs }
    }
}

fn read_input() -> Vec<Sequence> {
    let file_contents =
        fs::read_to_string("input/day09.txt")
        .expect("Failed to read input file!");

    file_contents.lines().map(|line_str| Sequence {
        nums: line_str
            .split_whitespace()
            .map(|num_str| num_str.parse().expect("Failed to parse history number!"))
            .collect()
    }).collect()
}

fn puzzle_one(histories: &Vec<Sequence>) -> isize {
    let mut summer: isize = 0;
    for history in histories {
        // compute sequence of difference, push them into a stack
        let mut difference_stack: Vec<Sequence> = vec![history.take_differences()];
        while !difference_stack.last().unwrap().nums.iter().all(|&diff| diff == 0) {
            difference_stack.push(difference_stack.last().unwrap().take_differences())
        }

        // final list of differences should be zero, make sure history is not constant
        difference_stack.pop();
        assert!(difference_stack.len() > 0, "History is constant!");

        // go through the stack, updating the differences with a new one following the sequence
        while difference_stack.len() > 1 {
            let curr_differences = difference_stack.pop().unwrap();
            let curr_last_val = *difference_stack.last().unwrap().nums.last().unwrap();

            difference_stack.last_mut().unwrap()
                .nums.push(curr_last_val + curr_differences.nums.last().unwrap());
        }

        // extrapolate the history into the future given the last difference sequence
        let last_history_num = *history.nums.last().unwrap();
        let last_difference  = *difference_stack.pop().unwrap().nums.last().unwrap();
        summer += last_history_num + last_difference;
    }

    return summer;
}

fn puzzle_two(histories: &Vec<Sequence>) -> isize {
    let mut summer: isize = 0;
    for history in histories {
        // compute sequence of difference, push them into a stack
        let mut difference_stack: Vec<Sequence> = vec![history.take_differences()];
        while !difference_stack.last().unwrap().nums.iter().all(|&diff| diff == 0) {
            difference_stack.push(difference_stack.last().unwrap().take_differences())
        }

        // final list of differences should be zero, make sure history is not constant
        difference_stack.pop();
        assert!(difference_stack.len() > 0, "History is constant!");

        // go through the stack, updating the differences with a new one following the sequence
        while difference_stack.len() > 1 {
            let curr_differences = difference_stack.pop().unwrap();
            let curr_first_val = *difference_stack.last().unwrap().nums.first().unwrap();

            difference_stack.last_mut().unwrap()
                .nums.insert(0, curr_first_val - curr_differences.nums.first().unwrap());
        }

        // extrapolate the history into the past given the last difference sequence
        let first_history_num = *history.nums.first().unwrap();
        let first_difference  = *difference_stack.pop().unwrap().nums.first().unwrap();
        summer += first_history_num - first_difference;
    }

    return summer;
}

fn main() {
    let histories = read_input();

    println!("Puzzle 1: {}", puzzle_one(&histories));
    println!("Puzzle 2: {}", puzzle_two(&histories));
}
