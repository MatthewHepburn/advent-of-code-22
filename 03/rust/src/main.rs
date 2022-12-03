use std::env;
use std::fs;
use core::str::Chars;
use core::str::Lines;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    // WTF this even means, I have no idea. But it works!
    let result = match &(&args[1])[..] {
        "--solve-a" => solve_a(),
        "--solve-b" => solve_b(),
        _ => panic!("Must provide --solve-a or --solve-b as first arg")
    };

    match result {
        Ok(val) => println!("{}", val),
        Err(e) => panic!("{}", e)
    }
}

struct Rucksack<'a> {
    pouch_size: usize,
    contents: Chars<'a>
}
type ItemType = char;
type ElfGroup<'a> = Vec<Rucksack<'a>>;


fn get_incorrect_item_type(mut rucksack: Rucksack) -> ItemType
{
    let mut seen: HashMap<ItemType, bool> = HashMap::new();

    for _i in 0..rucksack.pouch_size {
        // Record items seen in first pouch
        let item = rucksack.contents.next().unwrap();
        seen.insert(item, true);
    }

    for char in rucksack.contents {
        // Look for items in the second pouch that we saw in the first
        if seen.contains_key(&char) {
            return char
        }
    }

    panic!("No duplicate item found!")
}

fn get_common_item_type(mut elf_group: ElfGroup) -> ItemType
{
    let mut seen: HashMap<ItemType, i32> = HashMap::new();

    for item in elf_group.pop().unwrap().contents {
        // Record all items the first elf has
        seen.insert(item, 1);
    }

    for item in elf_group.pop().unwrap().contents {
        // Record all items common to first and second elf
        if seen.contains_key(&item) {
            seen.insert(item, 2);
        }
    }

    for item in elf_group.pop().unwrap().contents {
        if seen.get(&item) == Some(&2)  {
            // Item is common to all three elfs
            return item;
        }
    }

    panic!("No duplicate item found!")
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
        let rucksack = Rucksack{contents: line.chars(), pouch_size: line.len() / 2};
        output.push(rucksack);
    }

    return output;
}

fn solve_a() -> Result<i32, &'static str> {
    let input: String = load_input();
    let rucksacks = parse_input_as_rucksacks(input.lines());

    let mut priority_sum = 0;

    for rucksack in rucksacks {
        let item_type = get_incorrect_item_type(rucksack);
        priority_sum += get_item_type_priority(item_type);
    }

    return Ok(priority_sum);
}

fn solve_b() -> Result<i32, &'static str> {
    let input: String = load_input();
    let rucksacks = parse_input_as_rucksacks(input.lines());
    let mut elf_groups : Vec<ElfGroup> = Vec::new();

    let mut current_group : Vec<Rucksack> = Vec::new();
    let mut current_group_size = 0;
    for rucksack in rucksacks {
        current_group.push(rucksack);
        current_group_size += 1;

        if current_group_size == 3 {
            elf_groups.push(current_group);
            current_group = Vec::new();
            current_group_size = 0;
        }
    }

    let mut priority_sum = 0;
    for elf_group in elf_groups {
        let item_type = get_common_item_type(elf_group);
        priority_sum += get_item_type_priority(item_type);
    }

    return Ok(priority_sum);
}

fn is_example_mode() -> bool {
    let example_mode = env::var("AOC_EXAMPLE_MODE");
    if example_mode.is_err() {
        return false;
    }

    return example_mode.unwrap() == "1"
}

fn load_input() -> String {
    let filename = if is_example_mode() { "exampleInput.txt" } else { "input.txt" };
    let mut path = String::from("../");
    path.push_str(filename);

    return fs::read_to_string(path)
        .expect("Something went wrong reading the file");
}
