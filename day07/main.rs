use std::{collections::HashMap, fs, ops::Range};

#[derive(Debug, Clone)]
struct Command<'a> {
    executed_command: &'a str,
    index: usize,
    output: Vec<ListEntry>,
}

impl<'a> Command<'a> {
    pub fn new(executed_command: &'a str, index: usize, output: Vec<ListEntry>) -> Command<'a> {
        Command {
            executed_command,
            index,
            output,
        }
    }

    pub fn is_list_command(&self) -> bool {
        self.executed_command == "$ ls"
    }
}

#[derive(Debug, Clone)]
struct ListEntry {
    size: i32,
    name: String,
}

impl ListEntry {
    pub fn new(size: i32, name: String) -> ListEntry {
        ListEntry { size, name }
    }
}

struct FilePath {
    path: Vec<String>,
}

impl FilePath {
    pub fn root() -> FilePath {
        FilePath { path: vec![] }
    }

    pub fn go_back(&mut self) {
        if self.path.is_empty() {
            // Maybe just early return?
            panic!("Cannot go back any further");
        }

        self.path.pop();
    }

    pub fn cd(&mut self, target: String) {
        if target == ".." {
            self.go_back();
        } else if target == "/" {
            self.cd_root();
        } else {
            self.path.push(target);
        }
    }

    fn cd_root(&mut self) {
        self.path.clear();
    }

    pub fn pwd(&self) -> String {
        let prefix = "/";

        prefix.to_owned() + &self.path.join("/")
    }
}

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let commands = parse_input(contents.split("\n").into_iter().collect::<Vec<&str>>());

    let mut file_system = HashMap::new();
    // Iterate over commands and build a file tree from it
    let mut current_location = FilePath::root();
    for command in commands {
        if command.executed_command.starts_with("$ cd") {
            current_location.cd(command.executed_command.replace("$ cd ", ""));
        }

        if command.is_list_command() {
            let command_output = command.output;
            file_system.insert(current_location.pwd(), command_output);
        }
    }

    let mut path_to_file_size_map = HashMap::new();

    for path in file_system.keys() {
        path_to_file_size_map.insert(path.to_owned(), get_total_size(&file_system, path));
    }

    let updated_file_system = update_file_sizes(&mut file_system, &path_to_file_size_map);

    let mut file_size_sum_below_100k = 0;

    for size in path_to_file_size_map.values() {
        if *size < 100000 {
            file_size_sum_below_100k = file_size_sum_below_100k + *size;
        }
    }

    println!("Day 7 part 1: {:?}", file_size_sum_below_100k);

    let root_size: i32 = updated_file_system
        .get(&String::from("/"))
        .unwrap()
        .iter()
        .map(|entry| entry.size)
        .sum();

    // We have 70m units of free space we need 30m of units to solve the task which means the root needs to have 40m of free units
    let required_space = root_size - 40_000_000;

    let smallest_deletion = file_system
        .values()
        .map(|files| files.iter().map(|file| file.size))
        .flatten()
        .filter(|size| *size > required_space)
        .min();

    println!("Day 7 part 2: {}", smallest_deletion.unwrap());
}

fn get_total_size(file_system: &HashMap<String, Vec<ListEntry>>, current_path: &String) -> i32 {
    let mut file_size_sum = 0;
    for files in file_system.get(current_path) {
        for file in files {
            if file.size == 0 {
                let target_path = current_path.to_owned() + "/" + &file.name;
                if file_system.contains_key(&target_path) {
                    // Current file is a directory, get directory size
                    let dir_size = get_total_size(file_system, &target_path);

                    file_size_sum = file_size_sum + dir_size;
                }
            } else {
                file_size_sum = file_size_sum + file.size;
            }
        }
    }

    file_size_sum
}

fn update_file_sizes<'a>(
    file_system: &'a mut HashMap<String, Vec<ListEntry>>,
    path_to_file_size_map: &'a HashMap<String, i32>,
) -> &'a mut HashMap<String, Vec<ListEntry>> {
    for (path, files) in file_system.iter_mut() {
        for file in files {
            if file.size == 0 {
                let target_path;
                if path.to_owned() == "/" {
                    target_path = path.to_owned() + &file.name;
                } else {
                    target_path = path.to_owned() + "/" + &file.name;
                }
                file.size = match path_to_file_size_map.get(&target_path) {
                    None => panic!("Cannot find size for {:?}, {:?}", target_path, path),
                    Some(size) => *size,
                };
            }
        }
    }

    file_system
}

fn parse_input<'a>(input_lines: Vec<&'a str>) -> Vec<Command<'a>> {
    let commands = input_lines
        .iter()
        .enumerate()
        .filter(|(_, vec_line)| vec_line.starts_with("$"))
        .map(|(idx, vec_line)| Command::new(vec_line, idx, vec![]))
        .collect::<Vec<Command>>();

    commands
        .iter()
        .enumerate()
        .map(|(idx, command)| {
            if !command.is_list_command() {
                // If the command is not a list command we don't expect output
                command.clone()
            } else {
                Command::new(
                    command.executed_command,
                    command.index,
                    get_list_command_output(&commands, idx, &input_lines),
                )
            }
        })
        .collect()
}

fn get_list_command_output<'a>(
    commands: &Vec<Command>,
    current_command_index: usize,
    input_lines: &Vec<&'a str>,
) -> Vec<ListEntry> {
    match commands.get(current_command_index + 1) {
        // No next command so we take everything from current index to end
        None => get_lines_of_range(
            &input_lines,
            Range {
                start: commands.get(current_command_index).unwrap().index,
                end: input_lines.len(),
            },
        ),
        // Next command exists so we take everything from current command until start of next command
        Some(next_command) => get_lines_of_range(
            &input_lines,
            Range {
                start: commands.get(current_command_index).unwrap().index,
                end: next_command.index,
            },
        ),
    }
    .iter()
    .map(|list_line| {
        ListEntry::new(
            extract_size_from_list_output(list_line),
            extract_name_from_list_output(list_line),
        )
    })
    .collect()
}

fn extract_size_from_list_output(line: &str) -> i32 {
    let mut split = line.split(" ");
    match split.next() {
        None => panic!("No splitting possible"),
        Some(v) => match v.parse::<i32>() {
            Ok(size) => size,
            Err(_) => 0,
        },
    }
}

fn extract_name_from_list_output(line: &str) -> String {
    let mut split = line.split(" ");
    split.next();

    match split.next() {
        None => panic!("Cannot determine file name"),
        Some(v) => v.to_string(),
    }
}

fn get_lines_of_range<'a>(lines: &Vec<&'a str>, range: Range<usize>) -> Vec<&'a str> {
    lines
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx > range.start && *idx < range.end)
        .map(|(_, line)| line.to_owned())
        .collect::<Vec<&str>>()
}
