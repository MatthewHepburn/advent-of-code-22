use std::env;
use std::cmp::max;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "--solve-a" {
        let solution = solve_a();
        println!("{}", solution)
    } else if args[1] == "--solve-b" {
        let solution = solve_b();
        println!("{}", solution)
    } else {
        println!("Pass either --solve-a or --solve-b")
    }
}

fn solve_a() -> i32 {
    let calories = load_input();

    let mut current_calories = 0;
    let mut max_calories = 0;

    for calorie in calories {
        if calorie == -1 {
            // Reached end of current elf's inventory
            max_calories = max(current_calories, max_calories);
            current_calories = 0;
        } else {
            current_calories = current_calories + calorie
        }
    }

    return max_calories;
}

fn solve_b() -> i32 {
    let calories = load_input();

    let mut calorie_totals: Vec<i32> = Vec::new();
    let mut current_calories = 0;

    for calorie in calories {
        if calorie == -1 {
            // Reached end of current elf's inventory
            calorie_totals.push(current_calories);
            current_calories = 0;
        } else {
            current_calories = current_calories + calorie
        }
    }

    let mut max_1 = 0;
    let mut max_2 = 0;
    let mut max_3 = 0;
    for total in calorie_totals {
        if total >= max_1 {
            max_3 = max_2;
            max_2 = max_1;
            max_1 = total;
        } else if total >= max_2  {
            max_3 = max_2;
            max_2 = total;
        } else if total > max_3 {
            max_3 = total
        }
    }

    return max_1 + max_2 + max_3;
}

fn is_example_mode() -> bool {
    let example_mode = env::var("AOC_EXAMPLE_MODE");
    if example_mode.is_err() {
        return false;
    }

    return example_mode.unwrap() == "1"
}

fn load_input() -> Vec<i32> {
    let filename = if is_example_mode() { "exampleInput.txt" } else { "input.txt" };
    let mut path = String::from("../");
    path.push_str(filename);

    let contents : String = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let mut output: Vec<i32> = Vec::new();
    for line in contents.lines() {
        if line == "" {
            output.push(-1);
        } else {
            output.push(line.parse::<i32>().unwrap());
        }
    }

    return output;
}
