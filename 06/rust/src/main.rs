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


fn solve_a(input_filename: &str) -> ResultOrErr<usize> {
    let input_string = load_input(input_filename)?;
    let input: &str = input_string.split_whitespace().next().unwrap();
    let chars: Vec<char> = input.chars().collect();
    for i in 4..input.len() {
        let slice = &chars[i - 4..i];
        assert!(slice.len() == 4);
        let mut seen: HashMap<char, bool> = HashMap::new();
        for char in slice {
            seen.insert(*char, true);
        }
        if seen.len() == 4 {
            return Ok(i);
        }
    }

    return Err("No start of packet found".to_string());
}

fn solve_b(input_filename: &str) -> ResultOrErr<usize> {
    return Err("Not implemented".to_string())
}

fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
