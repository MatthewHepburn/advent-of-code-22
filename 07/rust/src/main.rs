extern crate core;

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
    seen_files: HashMap<String, i32>,
    dirs: HashMap<String, Dir>
}

impl FileSystem {
    fn get_current_dir_struct(self: &mut FileSystem) -> &mut Dir {
        let mut path = "".to_string();
        self.current_position.as_string(&mut path);

        let dir= self.dirs.get_mut(&path);
        return match dir {
            Some(dir) => dir,
            None => {
                println!("Error reading path '{}'", path);
                panic!("Logic error - directory not found")
            }
        }
    }

    fn process_command(self: &mut FileSystem, command: &Command) {
        match command {
            Command::Move(x) => self.do_move(x),
            Command::List(x) => self.do_list(x)
        }
    }

    fn do_move(self: &mut FileSystem, move_command: &MoveCommand) {
        self.chdir(move_command);
    }

    fn go_up(self: &mut FileSystem) {
        self.current_position.position_parts.pop();
    }

    fn go_to_root(self: &mut FileSystem) {
        self.current_position.position_parts.clear()
    }

    fn descend_into(self: &mut FileSystem, dir: &String) {
        self.current_position.position_parts.push(dir.clone());

        let mut path = "".to_string();
        self.current_position.as_string(&mut path);
        match self.dirs.get_mut(&path) {
            Some(_) => (),
            None => {
                let new_dir = Dir{files: HashMap::new()};
                self.dirs.insert(path, new_dir);
                return ();
            }
        }
    }

    fn chdir(self: &mut FileSystem, command: &MoveCommand) {
        println!("$ cd {}", command.target);
        if command.target == ".." {
            self.go_up()
        } else if command.target == "/" {
            self.go_to_root()
        } else {
            self.descend_into(&command.target)
        }

        let mut new_position: String = "".to_string();
        self.current_position.as_string(&mut new_position);
        println!("New location = {}", new_position)
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
                    self.see_file(name.clone(), absolute_filename, *size)
                }
            }
        }
    }

    fn see_file(self: &mut FileSystem, relative_filename: String, absolute_filename: String, size: i32) {
        println!("Saw file '{}' with size {}", absolute_filename, size);
        self.seen_files.insert(absolute_filename, size);
        self.get_current_dir_struct().files.insert(relative_filename, size);
    }

    fn seen_size(self: &FileSystem) -> i32 {
        let mut output = 0;
        for (file, size) in &self.seen_files {
            println!("Saw '{}' with size {}", file, size);
            output += size;
        }

        return output;
    }

    fn seen_size_minus_big_dirs(self: &FileSystem) -> i32 {
        let mut output = 0;
        for (path, dir) in &self.dirs {
            let mut size = self.get_dir_size(path.clone(), dir);

            if size <= 100000 {
                println!("Saw small dir '{}' with size {}", path, size);
                output += size;
            } else {
                println!("Ignored big dir '{}' with size {}", path, size);
            }
        }

        return output;
    }

    fn get_dir_size(self: &FileSystem, mut path: String, dir: &Dir) -> i32 {
        let mut output = dir.get_direct_file_size();

        if path != "/" {
            path.push_str("/");
        }
        for (other_path, other_dir) in self.dirs.iter() {
            if other_path == "/" {
                continue;
            }
            if !other_path.starts_with(&path) {
                continue;
            }

            println!("'{}' is a subdirectory of '{}'", other_path, path);

            // Only count other dirs direct contents here - since we're doing a prefix search we'll
            // also cover all of this dirs descendents
            output += other_dir.get_direct_file_size()
        }

        return output
    }
}

struct Dir {
    files: HashMap<String, i32>,
}

impl Dir {
    fn get_direct_file_size(self: &Dir) -> i32 {
        let mut output = 0;
        for (_, size) in &self.files {
            output += size;
        }

        return output;
    }
}

struct Position {
    position_parts: Vec<String>
}

impl Position {
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

    let start_dir: Dir = Dir{files: HashMap::new()};
    let mut dirs: HashMap<String, Dir> = HashMap::new();
    dirs.insert("/".to_string(), start_dir);
    let mut file_system: FileSystem = FileSystem {
        seen_files: HashMap::new(),
        current_position: Position {position_parts: Vec::new()},
        dirs
    };

    for command in commands {
        file_system.process_command(&command)
    }

    println!("-----FILE LISTING-----");
    for (path, _) in file_system.seen_files.iter() {
        println!("{}", path);
    }

    println!("-----DIR LISTING-----");
    for (path,dir) in file_system.dirs.iter() {
        println!("{} - {}", path, file_system.get_dir_size(path.clone(), dir));
    }

    return Ok(file_system.seen_size_minus_big_dirs());
}

fn solve_b(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let commands: Vec<Command> = parse_commands(input_string)?;

    let start_dir: Dir = Dir{files: HashMap::new()};
    let mut dirs: HashMap<String, Dir> = HashMap::new();
    dirs.insert("/".to_string(), start_dir);
    let mut file_system: FileSystem = FileSystem {
        seen_files: HashMap::new(),
        current_position: Position {position_parts: Vec::new()},
        dirs
    };

    for command in commands {
        file_system.process_command(&command)
    }

    let filesystem_size = 70000000;
    let required_space = 30000000;
    let root_dir = file_system.dirs.get("/").unwrap();
    let used_space = file_system.get_dir_size("/".to_string(), root_dir);
    let remaining_space = filesystem_size - used_space;
    let must_free = required_space - remaining_space;

    println!("Must free {}", must_free);

    let mut best_dir_size = required_space;
    for (path,dir) in file_system.dirs.iter() {
        let dir_size = file_system.get_dir_size(path.clone(), dir);
        if dir_size > must_free && dir_size < best_dir_size {
            best_dir_size = dir_size;
            println!("New best dir size {} for dir {}", best_dir_size, path)
        }
    }

    return Ok(best_dir_size);
}



fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
