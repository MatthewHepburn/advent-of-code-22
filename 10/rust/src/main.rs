extern crate core;

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


type Commands = Vec<Command>;
enum Command {
    Noop,
    Add(i32)
}

impl Command {
    fn get_duration(self : &Command) -> i32 {
        match self {
            Command::Noop => 1,
            Command::Add(_) => 2
        }
    }

    fn get_register_delta(self: &Command) -> i32 {
        match self {
            Command::Noop => 0,
            Command::Add(x) => *x
        }
    }

    fn print(self: &Command) {
        match self {
            Command::Noop => println!("noop"),
            Command::Add(x) => println!("addx {}", x)
        }
    }
}

fn parse_commands(input: String) -> ResultOrErr<Commands> {
    let mut commands : Commands = Vec::new();
    for line in input.lines() {
        if line == "" {
            continue;
        }

        let parts : Vec<&str> = line.split_whitespace().collect();
        let command = match parts[0] {
            "addx" => Command::Add(parts[1].parse().unwrap()),
            "noop" => Command::Noop,
            _ => panic!("Unexpected command")
        };
        
        commands.push(command);
    }

    return Ok(commands)
}

fn is_interesting_cycle_number(cycle: i32) -> bool {
    let mut x = cycle;
    while x > 0 {
        if x == 20 {
            return true
        }
        x = x - 40;
    }

    return false;
}

fn solve_a(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let commands: Commands = parse_commands(input_string)?;

    let mut register = 1;
    let mut counter = 1;

    let mut signal_sum = 0;

    for command in commands {
        for _ in 0..command.get_duration() {
            println!("{} - {}", counter, register);
            if is_interesting_cycle_number(counter) {
                let signal_strength = register * counter;
                signal_sum += signal_strength;
                println!("----------- sig_strength = {}", signal_strength);
            }

            counter += 1;
        }
        register += command.get_register_delta();
    }

    return Ok(signal_sum);
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
