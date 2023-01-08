use std::borrow::BorrowMut;
use std::fs;
use std::collections::HashSet;

#[derive(PartialEq)]
enum Command {
    LS,
    CD(String),
}
impl Command {
    // parse a line into a command
    fn parse(line: &str) -> Command {
        assert!(line.starts_with("$"), "Given line is not a command!");

        let space_split = line.split(" ").collect::<Vec<&str>>();
        assert!(space_split.len() > 1, "Invalid command, got just $!");

        match space_split[1] {
            "cd" => {
                assert!(space_split.len() == 3, "Invalid cd command!");
                Command::CD(space_split[2].to_string())
            },
            "ls" => {
                assert!(space_split.len() == 2, "Invalid ls command!");
                Command::LS
            },
            _    => { unreachable!("Unrecognized command!"); }
        }
    }
}

struct Directory {
    size:     usize,
    sub_dirs: Vec<Directory>
}

struct FileSystem {
    root_dir: Directory
}
impl FileSystem {
    fn dir_sizes(&self) -> Vec<usize> {
        let mut dir_sizes: Vec<usize> = Vec::new();
        let mut dirs_to_check: Vec<&Directory> = vec![&self.root_dir];

        while let Some(dir) = dirs_to_check.pop() {
            dir_sizes.push(dir.size);
            for sub_dir in &dir.sub_dirs {
                dirs_to_check.push(&sub_dir);
            }
        }
        return dir_sizes;
    }
}

// determine the contents of a newly cd-ed to directory
fn parse_directory(cmd_lines: &mut std::iter::Peekable<std::str::Lines>) -> Directory {
    // check that our first command after cd is an ls
    let first_cmd =
        Command::parse(cmd_lines.next()
        .expect("Ran out of command lines!"));

    match first_cmd {
        Command::LS     => {}, 
        Command::CD(..) => unreachable!("First command after cd to new directory was not ls!")
    };

    // collect the dump from the ls command
    let mut ls_lines: Vec<&str> = Vec::new();
    while !cmd_lines.peek().map_or_else(|| true, |line| line.starts_with('$')) {
        ls_lines.push(cmd_lines.next().unwrap());
    }

    // collect the files in the directory and keep track of subdir names
    let mut files_size: usize = 0;
    let mut sub_dirs_set: HashSet<String> = HashSet::new();
    for ls_line in ls_lines {
        let split_line = ls_line.split(" ").collect::<Vec<&str>>();
        assert!(split_line.len() == 2, "Bad format on an ls line!");

        match split_line[0] {
            "dir" => { sub_dirs_set.insert(split_line[1].to_string()); }
            _     => {
                let file_size = split_line[0].parse::<usize>().expect("Failed to parse file size!");
                files_size += file_size;
            }
        };
    }

    // continue parsing the commands, parsing the sub directories
    let mut sub_dirs: Vec<Directory> = Vec::new();
    while let Some(command) = cmd_lines.next().and_then(|line| Some(Command::parse(line))) {
        match command {
            Command::LS              => { unreachable!("Expected a cd call after listing the contents!"); }
            Command::CD(subdir_name) => {
                match subdir_name.as_str() {
                    ".." => { // cd to parent, meaning we should be finished with this diretory
                        assert!(sub_dirs_set.len() == 0, "Still had some directories to cd and ls!");
                        let dir_size = files_size + sub_dirs.iter().map(|dir| dir.size).sum::<usize>();
                        return Directory{ size: dir_size, sub_dirs: sub_dirs };
                    }
                    _    => { // cd to a sub directory, make sure we've seen it before
                        assert!(sub_dirs_set.remove(&subdir_name), "Tried to cd into an unknown directory!");
                        sub_dirs.push(parse_directory(cmd_lines.borrow_mut()));
                    }
                };
            }
        };
    }

    assert!(sub_dirs_set.len() == 0, "Still had some directories to cd and ls!");
    let dir_size = files_size + sub_dirs.iter().map(|dir| dir.size).sum::<usize>();
    return Directory{ size: dir_size, sub_dirs: sub_dirs };

}

fn read_input() -> FileSystem {
    let file_contents =
        fs::read_to_string("input/day07.txt")
        .expect("Failed to read input file!");

    let mut cmd_lines = file_contents.lines().peekable();
    let first_cmd: Command = Command::parse(cmd_lines.next().unwrap());
    match first_cmd {
        Command::LS       => unreachable!("Got an ls for the first command!"),
        Command::CD(name) => assert!(name == "/", "First cd is not to the root directory!")
    };

    FileSystem {
        root_dir: parse_directory( cmd_lines.borrow_mut())
    }
}

static MAX_DIR_SIZE: usize = 100000;
fn puzzle_one(file_sys: &FileSystem) -> usize {
    file_sys.dir_sizes().iter()
        .filter(|size| **size <= MAX_DIR_SIZE)
        .sum()
}

static MAX_FS_SIZE : usize = 70000000;
static UPDATE_SIZE : usize = 30000000;
fn puzzle_two(file_sys: &FileSystem) -> usize {
    let to_delete: usize = UPDATE_SIZE - (MAX_FS_SIZE - file_sys.root_dir.size);
    assert!(to_delete > 0, "We don't have to remove anything to fit the update!");

    *file_sys.dir_sizes().iter()
        .filter(|size| **size >= to_delete)
        .min().unwrap()
}

fn main() {
    let file_system: FileSystem = read_input();

    println!("Puzzle 1: {}", puzzle_one(&file_system));
    println!("Puzzle 2: {}", puzzle_two(&file_system));
}