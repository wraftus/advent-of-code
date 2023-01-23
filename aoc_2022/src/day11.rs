use std::fs;
use std::cmp::{min, max};

#[derive(Clone)]
enum Operation {
    Add(Option<usize>),
    Mlt(Option<usize>),
}
impl Operation {
    fn parse(op_text: &str) -> Result<Operation, &'static str> {
        let split_text = op_text.split(" ").collect::<Vec<&str>>();
        if split_text.len() != 2 {
            return Err("Poorly formed operation text!");
        }

        let num = match split_text[1] {
            "old" => None,
            _     => Some(split_text[1].parse::<usize>()
                          .expect("Failed to parse operation's number!"))
        };
        match split_text[0] {
            "+"   => Ok(Operation::Add(num)),
            "*"   => Ok(Operation::Mlt(num)),
            _     => Err("Unrecognized operation!")
        }
    }

    fn apply(&self, worry: usize) -> usize {
        match self {
            Operation::Add(num) => match num {
                None      => 2*worry,
                Some(num) => worry + num
            },
            Operation::Mlt(num) => match num {
                None      => worry * worry,
                Some(num) => worry * num
            }
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items:  Vec<usize>,
    oprtn:  Operation,
    to_mod: usize,
    t_idx:  usize,
    f_idx:  usize
}
impl Monkey {
    fn parse(lines: &Vec<&str>) -> Result<Monkey, &'static str> {
        let items_fluf: &'static str = "  Starting items: ";
        let oprtn_fluf: &'static str = "  Operation: new = old ";
        let tomod_fluf: &'static str = "  Test: divisible by ";
        let tridx_fluf: &'static str = "    If true: throw to monkey ";
        let flidx_fluf: &'static str = "    If false: throw to monkey ";

        if lines.len() != 6 {
            return Err("Bad number of lines for monkey!");
        } else if !lines[1].starts_with(items_fluf) {
            return Err("Bad monkey's items fluf text!");
        } else if !lines[2].starts_with(oprtn_fluf) {
            return Err("Bad monkey's operation fluf text!");
        } else if !lines[3].starts_with(tomod_fluf) {
            return Err("Bad monkey's test fluf text!");
        } else if !lines[4].starts_with(tridx_fluf) {
            return Err("Bad monkey truth index fluf text!");
        } else if !lines[5].starts_with(flidx_fluf) {
            return Err("Bad monkey false index fluf text!");
        }

        let starting_items =
            lines[1].replace(items_fluf, "")
            .split(", ")
            .map(|worry| worry.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let oprtn  = Operation::parse(lines[2].replace(oprtn_fluf, "").as_str()).unwrap();
        let to_mod = lines[3].replace(tomod_fluf, "").as_str().parse::<usize>().unwrap();
        let t_idx  = lines[4].replace(tridx_fluf, "").as_str().parse::<usize>().unwrap();
        let f_idx  = lines[5].replace(flidx_fluf, "").as_str().parse::<usize>().unwrap();

        Ok(Monkey {
            items: starting_items,
            oprtn,
            to_mod,
            t_idx,
            f_idx
        })
    }

}

fn read_input() -> Vec<Monkey> {
    let file_contents =
        fs::read_to_string("input/day11.txt")
        .expect("Failed to read input file!");

    file_contents.lines().fold(vec![Vec::new()], |mut monkey_grp, line| {
        if line.is_empty() {
            monkey_grp.push(Vec::new());
        } else {
            monkey_grp.last_mut().unwrap().push(line);
        }
        return monkey_grp;
    }).iter().map(|monkey_lines| {
        Monkey::parse(monkey_lines).unwrap()
    }).collect::<Vec<Monkey>>()
}

fn monkey_business(monkeys: &Vec<Monkey>, num_rounds: usize, worry_div: usize) -> usize {
    let mut inspect_counts = monkeys.iter().map(|_| 0).collect::<Vec<usize>>();
    let mut monkeys = monkeys.iter().map(|monkey| monkey.clone()).collect::<Vec<Monkey>>();

    let lcd = monkeys.iter().fold(1, |acc, monkey| acc*monkey.to_mod);

    for _ in 0..num_rounds {
        for idx in 0..monkeys.len() {
            let mut to_throw: Vec<(usize, usize)> = Vec::new();
            {
                let monkey = monkeys.get_mut(idx).unwrap();
                inspect_counts[idx] += monkey.items.len();

                for worry in monkey.items.drain(..) {
                    let worry = monkey.oprtn.apply(worry) / worry_div;

                    let other_idx;
                    if worry % monkey.to_mod == 0 {
                        other_idx = monkey.t_idx;
                    } else {
                        other_idx = monkey.f_idx;
                    }
                    to_throw.push((worry, other_idx));
                }
            }

            for (worry, idx) in to_throw.drain(..) {
                let worry = worry % lcd;
                monkeys.get_mut(idx).unwrap().items.push(worry);
            }
        }
    }

    let mut fst_max = max(inspect_counts[0], inspect_counts[1]);
    let mut snd_max = min(inspect_counts[0], inspect_counts[1]);
    for count in &inspect_counts[2..] {
        if *count > fst_max {
            snd_max = fst_max;
            fst_max = *count;
        }
        else if *count > snd_max {
            snd_max = *count;
        }
    }
    fst_max * snd_max
}

fn puzzle_one(monkeys: &Vec<Monkey>) -> usize {
    monkey_business(monkeys, 20, 3)
}

fn puzzle_two(monkeys: &Vec<Monkey>) -> usize {
    monkey_business(monkeys, 10000, 1)
}

fn main() {
    let monkeys: Vec<Monkey> = read_input();

    println!("Puzzle 1: {}", puzzle_one(&monkeys));
    println!("Puzzle 2: {}", puzzle_two(&monkeys));
}