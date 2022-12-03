use std::env;
use std::fs;
use core::str::Lines;
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
#[derive(Clone,Copy)]
struct Rucksack<'a> {
    pouch_size: usize,
    contents: &'a str
}
type ItemType = char;
type ElfGroup<'a> = [Rucksack<'a>; 3];


fn get_incorrect_item_type(rucksack: Rucksack) -> ResultOrErr<ItemType>
{
    let mut seen: HashMap<ItemType, bool> = HashMap::new();
    let mut index = 0;

    for item in rucksack.contents.chars() {
        if index < rucksack.pouch_size {
            // We're in the first pouch - record items seen
            seen.insert(item, true);
        } else {
            // We're in the second pouch - look for items that we saw in the first
            if seen.contains_key(&item) {
                return Ok(item);
            }
        }
        index += 1;
    }

    return Err("No duplicate item found!".to_string());
}

fn get_common_item_type(elf_group: ElfGroup) -> ResultOrErr<ItemType>
{
    let mut seen: HashMap<ItemType, i32> = HashMap::new();

    for item in (&elf_group[0]).contents.chars() {
        // Record all items the first elf has
        seen.insert(item, 1);
    }

    for item in (&elf_group[1]).contents.chars() {
        // Record all items common to first and second elf
        if seen.contains_key(&item) {
            seen.insert(item, 2);
        }
    }

    for item in (&elf_group[2]).contents.chars() {
        if seen.get(&item) == Some(&2)  {
            // Item is common to all three elfs
            return Ok(item);
        }
    }

    return Err("No common item found!".to_string())
}


fn get_item_type_priority(item_type: ItemType) -> i32
{
    let code_point =  item_type as i32;
    return if item_type.is_uppercase() {
        let reference_point = 'A' as i32;
        code_point - reference_point + 27
    } else {
        let reference_point = 'a' as i32;
        code_point - reference_point + 1
    }
}

fn parse_input_as_rucksacks(input: Lines) -> Vec<Rucksack>
{
    let mut output: Vec<Rucksack> = Vec::new();
    for line in input {
        if line == "" {
            continue
        }
        let rucksack = Rucksack{contents: line, pouch_size: line.len() / 2};
        output.push(rucksack);
    }

    return output;
}

fn solve_a(input_filename: &str) -> ResultOrErr<i32> {
    let input: String = load_input(input_filename)?;
    let rucksacks = parse_input_as_rucksacks(input.lines());

    let mut priority_sum = 0;

    for rucksack in rucksacks {
        let item_type = get_incorrect_item_type(rucksack)?;
        priority_sum += get_item_type_priority(item_type);
    }

    return Ok(priority_sum);
}

fn solve_b(input_filename: &str) -> ResultOrErr<i32> {
    let input: String = load_input(input_filename)?;
    let rucksacks = parse_input_as_rucksacks(input.lines());
    let mut elf_groups : Vec<ElfGroup> = Vec::new();

    let mut current_group : [Rucksack; 3] = [Rucksack{contents: "", pouch_size: 0}; 3];
    let mut current_group_size = 0;
    for rucksack in rucksacks {
        current_group[current_group_size] = rucksack;
        current_group_size += 1;

        if current_group_size == 3 {
            elf_groups.push(current_group);
            current_group = [Rucksack{contents: "", pouch_size: 0}; 3];
            current_group_size = 0;
        }
    }

    let mut priority_sum = 0;
    for elf_group in elf_groups {
        let item_type = get_common_item_type(elf_group)?;
        priority_sum += get_item_type_priority(item_type);
    }

    return Ok(priority_sum);
}

fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
