use std::env;
use std::fs;
use core::str::Lines;

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
struct ElfAssignment {
    lower: i32,
    upper: i32
}

impl ElfAssignment {
    fn fully_contains(&self, other: &ElfAssignment) -> bool {
        return self.lower <= other.lower && self.upper >= other.upper;
    }

    fn includes(&self, section:i32) -> bool {
        return self.lower <= section && self.upper >= section;
    }

    fn intersects(&self, other: &ElfAssignment) -> bool {
        if self.includes(other.lower) || self.includes(other.upper) {
            return true;
        }
        if self.fully_contains(other) {
            return true
        }
        if other.fully_contains(self) {
            return true
        }

        return false;
    }
}

struct ElfPair {
    assignments: [ElfAssignment; 2]
}

fn parse_assignment(input: &str) -> ElfAssignment
{
    let parts: Vec<&str> = input.split('-').collect();
    return ElfAssignment{
        lower: parts[0].parse::<i32>().unwrap(),
        upper: parts[1].parse::<i32>().unwrap(),
    }
}

fn parse_input(input: Lines) -> Vec<ElfPair>
{
    let mut output: Vec<ElfPair> = Vec::new();
    for line in input {
        if line == "" {
            continue
        }
        let assignment_strings : Vec<&str> = line.split(',').collect();
        let elf_pair = ElfPair{assignments: [
            parse_assignment(assignment_strings[0]), parse_assignment(assignment_strings[1])
        ]};
        output.push(elf_pair);
    }

    return output;
}

fn solve_a(input_filename: &str) -> ResultOrErr<i32> {
    let input: String = load_input(input_filename)?;
    let elf_pairs = parse_input(input.lines());

    let mut fully_contained = 0;
    for elf_pair in elf_pairs {
        if elf_pair.assignments[0].fully_contains(&elf_pair.assignments[1]) ||
            elf_pair.assignments[1].fully_contains(&elf_pair.assignments[0]) {
            fully_contained += 1;
        }
    }

    return Ok(fully_contained);
}

fn solve_b(input_filename: &str) -> ResultOrErr<i32> {
    let input: String = load_input(input_filename)?;
    let elf_pairs = parse_input(input.lines());

    let mut overlapping_pairs = 0;
    for elf_pair in elf_pairs {
        if elf_pair.assignments[0].intersects(&elf_pair.assignments[1]) {
            overlapping_pairs += 1;
        }
    }

    return Ok(overlapping_pairs);
}

fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
