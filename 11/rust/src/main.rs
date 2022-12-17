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

/**
Monkey 6:
  Starting items: 95, 79, 79, 68, 78
  Operation: new = old + 3
  Test: divisible by 3
    If true: throw to monkey 1
    If false: throw to monkey 3
**/

struct Item {
    worry_level: i32
}

struct Monkey {
    index: usize,
    items: Vec<Item>,
    test_divisor: i32,
    true_target: usize,
    false_target: usize,
    operation: Operation,
    items_inspected: i32
}

impl Monkey {
    fn print(self: &Monkey) {
        println!("Monkey {}:", self.index);

        print!("  Starting items: ");
        for item in &self.items {
            print!("{} ", item.worry_level)
        }
        println!("");

        print!("  Operation: ");
        self.operation.print();
        println!("");

        println!("  Test: divisible by {}", self.test_divisor);
        println!("    If true: throw to monkey {}", self.true_target);
        println!("    If false: throw to monkey {}", self.false_target);
    }
}

struct Operation {
    power: u32,
    add: i32,
    multiply: i32
}

impl Operation {
    fn from_string(input: &str) -> Operation {
        let end: &str = input.strip_prefix("  Operation: new = old ").unwrap();
        if end == "* old" {
            return Operation{power: 2, add: 0, multiply: 1}
        }
        let parts: Vec<&str> = end.split_whitespace().collect();
        if parts[0] == "+" {
            return Operation{power: 1, multiply: 1, add: parts[1].parse().unwrap()};
        }
        if parts[0] == "*" {
            return Operation{power: 1, add: 0, multiply: parts[1].parse().unwrap()};
        }

        panic!("Could not parse operation");
    }

    fn print(self: &Operation) {
        print!("{} * (old^{} + {})", self.multiply, self.power, self.add)
    }

    fn perform(self: &Operation, worry_level: i32) -> i32 {
        (worry_level.pow(self.power) + self.add) * self.multiply
    }
}

type Monkeys = Vec<Monkey>;

fn parse_monkeys(input: String) -> ResultOrErr<Monkeys> {
    let mut monkeys : Monkeys = Vec::new();

    let mut monkey_items : Vec<Item> = Vec::new();
    let mut monkey_test_divisor = 0;
    let mut monkey_true_target = 0;
    let mut monkey_false_target = 0;
    let mut monkey_operation: Operation = Operation{add: 0, power: 1, multiply: 1};
    let mut monkey_index = 0;

    for line in input.lines() {
        if line == "" {
            let monkey = Monkey{
                index: monkey_index,
                items: monkey_items,
                test_divisor: monkey_test_divisor,
                true_target: monkey_true_target,
                false_target: monkey_false_target,
                operation: monkey_operation,
                items_inspected: 0
            };
            monkeys.push(monkey);
            monkey_items = Vec::new();
            monkey_operation = Operation{add: 0, power: 1, multiply: 1};
            monkey_index += 1;
            continue;
        }


        let parts : Vec<&str> = line.split_whitespace().collect();

        if parts[0] == "Starting" {
            for i in 2..parts.len() {
                monkey_items.push(Item{worry_level: parts[i].replace(",", "").parse().unwrap()});
            }
        } else if parts[0] == "Operation:" {
            monkey_operation = Operation::from_string(line);
        } else if parts[0] == "Test:" {
            monkey_test_divisor = parts[3].parse().unwrap();
        } else if parts[0] == "If" && parts[1] == "true:" {
            monkey_true_target = parts[5].parse().unwrap();
        } else if parts[0] == "If" && parts[1] == "false:" {
            monkey_false_target = parts[5].parse().unwrap();
        } else if parts[0] == "Monkey" {
            // Do nothing - we can deduce the index from the order
        } else {
            panic!("Failed to parse line '{}', part[0] = '{}'", line, parts[0])
        }
    }

    return Ok(monkeys)
}

fn solve_a(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let mut monkeys: Monkeys = parse_monkeys(input_string)?;

    for monkey in &monkeys {
        monkey.print();
        println!("")
    }

    for round in 1..21 {
        println!("--- Round {} ---", round);
        for index in 0..monkeys.len() {
            println!("Monkey {}:", monkeys[index].index);
            while monkeys[index].items.len() > 0 {
                let mut item = monkeys[index].items.remove(0);
                println!("  Monkey inspects an item with a worry level of {}.", item.worry_level);
                item.worry_level = monkeys[index].operation.perform(item.worry_level);
                println!("    Worry level is increased to {}.", item.worry_level);
                item.worry_level = item.worry_level / 3;
                println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", item.worry_level);

                let divisible = item.worry_level % monkeys[index].test_divisor == 0;
                let target_monkey = if divisible { monkeys[index].true_target } else { monkeys[index].false_target };
                if divisible {
                    println!("    Current worry level is divisible by {}.", monkeys[index].test_divisor)
                } else {
                    println!("    Current worry level is not divisible by {}.", monkeys[index].test_divisor)
                }

                println!("    Item with worry level {} is thrown to monkey {}.", item.worry_level, target_monkey);
                monkeys[target_monkey].items.push(item);
                monkeys[index].items_inspected += 1;
            }
        }

        println!("--Round ends!");
        println!("After round {}, the monkeys are holding items with these worry levels:", round);
        for monkey in &monkeys {
            print!("Monkey {}: ", monkey.index);
            for item in &monkey.items {
                print!("{} ", item.worry_level)
            }
            println!("");
        }
    }

    let mut top_monkey_score = 0;
    let mut second_monkey_score = 0;

    for monkey in &monkeys {
        if monkey.items_inspected >= top_monkey_score {
            second_monkey_score = top_monkey_score;
            top_monkey_score = monkey.items_inspected
        } else if monkey.items_inspected > second_monkey_score {
            second_monkey_score = monkey.items_inspected
        }
    }

    return Ok(top_monkey_score * second_monkey_score);
}

fn solve_b(input_filename: &str) -> ResultOrErr<i32> {
    return Err("Not implemented".to_string())
}



fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
