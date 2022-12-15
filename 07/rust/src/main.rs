use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = &(&args[2])[..];

    let result = match &(&args[1])[..] {
        "--solve-a" => solve_a(input_filename),
        "--solve-b" => solve_b(input_filename),
        _ => panic!("Must provide --solve-a or --solve-b as first arg")
    };

    match result {
        Ok(val) => println!("{}", val),
        Err(e) => panic!("{}", e)
    }
}

type ResultOrErr<X> = Result<X, String>;

struct FileSystem {
    current_position: Position,
    seen_files: HashMap<String, i32>
}

impl FileSystem {
    fn process_command(self: &mut FileSystem, command: &Command) {
        match command {
            Command::Move(x) => self.do_move(x),
            Command::List(x) => self.do_list(x)
        }
    }

    fn do_move(self: &mut FileSystem, move_command: &MoveCommand) {
        self.current_position.chdir(move_command);
    }

    fn do_list(self: &mut FileSystem, list_command: &ListCommand) {
        println!("Doing list");
        let mut work_dir: String = "".to_string();
        self.current_position.as_string(&mut work_dir);
        for content in list_command.result.iter()  {
            match content {
                ListResult::Dir(_) => (),
                ListResult::File(name, size) => {
                    let mut absolute_filename = work_dir.clone();
                    if absolute_filename != "/" {
                        absolute_filename.push_str("/");
                    }
                    absolute_filename.push_str(name);
                    self.see_file(absolute_filename, *size)
                }
            }
        }
    }

    fn see_file(self: &mut FileSystem, absolute_filename: String, size: i32) {
        println!("Saw file '{}' with size {}", absolute_filename, size);
        self.seen_files.insert(absolute_filename, size);
    }

    fn seen_size(self: &FileSystem) -> i32 {
        let mut output = 0;
        for (file, size) in &self.seen_files {
            println!("Saw '{}' with size {}", file, size);
            output += size;
        }

        return output;
    }
}

struct Position {
    position_parts: Vec<String>
}

impl Position {
    fn go_up(self: &mut Position) {
        self.position_parts.pop();
    }

    fn go_to_root(self: &mut Position) {
        self.position_parts.clear()
    }

    fn descend_into(self: &mut Position, dir: &String) {
        self.position_parts.push(dir.clone())
    }

    fn chdir(self: &mut Position, command: &MoveCommand) {
        println!("$ cd {}", command.target);
        if command.target == ".." {
            self.go_up()
        } else if command.target == "/" {
            self.go_to_root()
        } else {
            self.descend_into(&command.target)
        }

        let mut new_position: String = "".to_string();
        self.as_string(&mut new_position);
        println!("New location = {}", new_position)
    }

    fn as_string(self: &mut Position, output: &mut String) {
        output.clear();
        for part in self.position_parts.iter() {
            output.push_str("/");
            output.push_str(&part);
        }

        if output == "" {
            output.push_str("/");
        }
    }
}

struct MoveCommand {
    target: String
}

struct ListCommand {
    result: Vec<ListResult>
}

enum Command {
    Move(MoveCommand),
    List(ListCommand)
}

enum ListResult {
    Dir(String),
    File(String, i32)
}

fn parse_commands(input: String) -> ResultOrErr<Vec<Command>> {
    let mut output:Vec<Command> = Vec::new();
    let mut last_ls: Vec<ListResult> = Vec::new();
    let mut last_ls_empty = true;

    for line in input.lines() {
        if line == "" {
            continue;
        }
        if line.starts_with('$') {
            // Encountered a command - if we were reading an ls, wrap it up
            if !last_ls_empty {
                output.push(Command::List(ListCommand{result: last_ls}));
                last_ls = Vec::new();
                last_ls_empty = true;
            }

            if line.starts_with("$ cd ") {
                let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
                output.push(Command::Move(MoveCommand{target: parts[2].to_string()}))
            }
        } else {
            // Must be ls output
            let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
            if parts[0] == "dir" {
                last_ls.push(ListResult::Dir(parts[1].to_string()))
            } else {
                let size: i32 = parts[0].parse().unwrap();
                let name: String = parts[1].to_string();
                last_ls.push(ListResult::File(name, size));
            }

            last_ls_empty = false;
        }
    }

    // We might have been reading an ls when we ran out of lines
    if !last_ls_empty {
        output.push(Command::List(ListCommand { result: last_ls }));
    }

    return Ok(output)
}


fn solve_a(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let commands: Vec<Command> = parse_commands(input_string)?;
    let mut file_system: FileSystem = FileSystem {
        seen_files: HashMap::new(),
        current_position: Position {position_parts: Vec::new()}
    };

    for command in commands {
        file_system.process_command(&command)
    }

    return Ok(file_system.seen_size());
}

fn solve_b(input_filename: &str) -> ResultOrErr<i32> {
    return Err("Not implemented".to_string());
}


fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
