use std::env;
use std::fs;

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

type Crate = char;
#[derive(Clone)]
struct Stack {
    crates: Vec<Crate>
}
struct CargoArea {
    stacks: Vec<Stack>
}

impl CargoArea {
    fn output(&self) {
        let mut stack_index = 1;
        for stack in &self.stacks {
            print!("stack {}: ", stack_index);
            for some_crate in &stack.crates {
                print!("[{}] ", some_crate);
            }
            println!("---");
            stack_index = stack_index + 1;
        }
    }
}

impl Stack {
    fn take_top_n_one_by_one(&mut self, n: i32) -> ResultOrErr<Vec<Crate>> {
        let mut output : Vec<Crate> = Vec::new();
        for _i in 0..n {
            let this_crate = match self.crates.pop() {
                Some(x) => Ok(x),
                None => Err("Tried to take from empty stack".to_string())
            }?;
            output.push(this_crate);
        }
        assert!(n == output.len().try_into().unwrap());
        return Ok(output);
    }

    fn add_crate(&mut self, new_crate: Crate) {
        self.crates.push(new_crate);
    }
}

struct Move {
    crate_count: i32,
    from_stack: usize,
    to_stack: usize
}

impl Move {
    fn output(&self) {
        println!("move {} from {} to {}", self.crate_count, self.from_stack, self.to_stack)
    }
}

struct Problem {
    cargo_area: CargoArea,
    move_list: Vec<Move>
}

fn parse_problem(input: String) -> ResultOrErr<Problem>
{
    let cargo_area = CargoArea{stacks: Vec::new()};
    let mut problem = Problem{cargo_area, move_list: Vec::new()};
    // First pass - work out how many stacks we have
    for line in input.lines() {
        if line == "" {
            continue
        }
        if line.starts_with(" 1 ") {
            for _stack in line.split_whitespace() {
                problem.cargo_area.stacks.push(
                    Stack{crates: Vec::new()}
                )
            }
            continue
        }
        if line.starts_with("move ") {
            let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
            problem.move_list.push(Move{
                crate_count: parts[1].parse().unwrap(),
                from_stack: parts[3].parse().unwrap(),
                to_stack: parts[5].parse().unwrap(),

            });
            continue
        }
    }

    // Second pass - work out how many stacks we have
    for line in input.lines() {
        if line == "" {
            continue
        }
        if line.starts_with(" 1 ") {
            break
        }
        let mut stack_index = 0;
        for stack in &mut problem.cargo_area.stacks {
            let string_pos = 1 + (stack_index * 4);
            let this_crate = line.chars().collect::<Vec<char>>()[string_pos];
            if this_crate != ' ' {
                stack.crates.push(this_crate);
            }
            stack_index += 1;
        }
    }

    for stack in &mut problem.cargo_area.stacks {
        stack.crates.reverse();
    }

    return Ok(problem);
}

fn make_move_9000(elf_move: &Move, cargo_area: &mut CargoArea) -> ResultOrErr<bool> {
    elf_move.output();

    let from_stack = &mut cargo_area.stacks[elf_move.from_stack - 1];

    for moved_crate in from_stack.take_top_n_one_by_one(elf_move.crate_count)? {
        let to_stack = &mut cargo_area.stacks[elf_move.to_stack - 1];
        to_stack.crates.push(moved_crate);
    }

    cargo_area.output();

    return Ok(true)
}

fn make_move_9001(elf_move: &Move, cargo_area: &mut CargoArea) -> ResultOrErr<bool> {
    elf_move.output();
    let mut crate_count = 0;
    let mut crates: Vec<Crate> = Vec::new();
    while crate_count < elf_move.crate_count {
        let this_crate = cargo_area.stacks[elf_move.from_stack - 1].crates.pop().ok_or("Could not pop".to_string())?;
        crates.push(this_crate);
        crate_count += 1;
    }

    crates.reverse();

    for this_crate in crates {
        cargo_area.stacks[elf_move.to_stack - 1].crates.push(this_crate);
    }

    cargo_area.output();

    return Ok(true)
}


fn solve_a(input_filename: &str) -> ResultOrErr<String> {
    let input: String = load_input(input_filename)?;
    let mut problem = parse_problem(input)?;

    problem.cargo_area.output();

    for elf_move in problem.move_list {
        make_move_9000(&elf_move, &mut problem.cargo_area)?;
    }

    let mut output: String = "".to_string();
    for mut stack in problem.cargo_area.stacks {
        output.push(stack.take_top_n_one_by_one(1)?[0])
    }

    return Ok(output);
}

fn solve_b(input_filename: &str) -> ResultOrErr<String> {
    let input: String = load_input(input_filename)?;
    let mut problem = parse_problem(input)?;

    problem.cargo_area.output();

    for elf_move in problem.move_list {
        make_move_9001(&elf_move, &mut problem.cargo_area)?;
    }

    let mut output: String = "".to_string();
    for mut stack in problem.cargo_area.stacks {
        output.push(stack.take_top_n_one_by_one(1)?[0])
    }

    return Ok(output);
}

fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
