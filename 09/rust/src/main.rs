extern crate core;

use std::collections::HashSet;
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

struct Vector {
    x: i32,
    y: i32
}

struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn get_as_string(self: &Position, output: &mut String) {
        output.clear();
        output.push_str(&self.x.to_string());
        output.push(',');
        output.push_str(&self.y.to_string())
    }

    fn get_vector_from(self: &Position, other: &Position) -> Vector {
        return Vector{
            x: other.x - self.x,
            y: other.y - self.y
        }
    }
}

struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    fn move_head(self: &mut Rope, x_offset: i32, y_offset: i32) {
        self.head.x += x_offset;
        self.head.y += y_offset;
    }

    fn move_tail(self: &mut Rope) {
        let tail_to_head: Vector = self.tail.get_vector_from(&self.head);
        if tail_to_head.x.abs() + tail_to_head.y.abs() < 2 {
            // Too close to do anything
            return;
        }
        if tail_to_head.y == 0 && tail_to_head.x == 2 {
            self.tail.x += 1;
            return;
        } else if tail_to_head.y == 0 && tail_to_head.x == -2 {
            self.tail.x -= 1;
            return;
        } else if tail_to_head.y == 2 && tail_to_head.x == 0 {
            self.tail.y += 1;
            return;
        } else if tail_to_head.y == -2 && tail_to_head.x == 0 {
            self.tail.y -= 1;
            return;
        }

        if tail_to_head.x.abs() + tail_to_head.y.abs() < 3 {
            // Too close to do anything - must be diagonally adjacent
            return;
        }

        // Check for diagonal moves
        if tail_to_head.x > 0 && tail_to_head.y > 0 {
            self.tail.x +=1;
            self.tail.y +=1;
            return;
        } else if tail_to_head.x > 0 && tail_to_head.y < 0 {
            self.tail.x +=1;
            self.tail.y -=1;
            return;
        } else if tail_to_head.x < 0 && tail_to_head.y > 0 {
            self.tail.x -=1;
            self.tail.y +=1;
            return;
        } else if tail_to_head.x < 0 && tail_to_head.y < 0 {
            self.tail.x -=1;
            self.tail.y -=1;
            return;
        }

        println!("Vector: x:{}, y:{}", tail_to_head.x, tail_to_head.y);
        panic!("No move found - unexpected position");
    }
}

struct LongRope {
    head: Position,
    tails: Vec<Position>,
}

impl LongRope {
    fn move_head(self: &mut LongRope, x_offset: i32, y_offset: i32) {
        self.head.x += x_offset;
        self.head.y += y_offset;
    }

    fn move_tails(self: &mut LongRope) {
        let mut head = &self.head;
        for tail in &mut self.tails {
            let tail_to_head: Vector = tail.get_vector_from(&head);
            if tail_to_head.x.abs() + tail_to_head.y.abs() < 2 {
                // Too close to do anything
                head = &&tail;
                return;
            }
            if tail_to_head.y == 0 && tail_to_head.x == 2 {
                tail.x += 1;
            } else if tail_to_head.y == 0 && tail_to_head.x == -2 {
                tail.x -= 1;
            } else if tail_to_head.y == 2 && tail_to_head.x == 0 {
                tail.y += 1;
            } else if tail_to_head.y == -2 && tail_to_head.x == 0 {
                tail.y -= 1;
            }

            if tail_to_head.x.abs() + tail_to_head.y.abs() < 3 {
                // Too close to do anything - must be diagonally adjacent
                head = tail;
                continue;
            }

            // Check for diagonal moves
            if tail_to_head.x > 0 && tail_to_head.y > 0 {
                tail.x +=1;
                tail.y +=1;
            } else if tail_to_head.x > 0 && tail_to_head.y < 0 {
                tail.x +=1;
                tail.y -=1;
            } else if tail_to_head.x < 0 && tail_to_head.y > 0 {
                tail.x -=1;
                tail.y +=1;
            } else if tail_to_head.x < 0 && tail_to_head.y < 0 {
                tail.x -=1;
                tail.y -=1;
            }

            head = tail;
        }
    }
}


type Commands = Vec<Command>;
enum Command {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Command {
    fn get_steps(self: &Command) -> i32 {
        return match self {
            Command::Up(x) => *x,
            Command::Down(x) => *x,
            Command::Left(x) => *x,
            Command::Right(x) => *x,
        }
    }

    fn get_x_offset(self: &Command) -> i32 {
        return match self {
            Command::Up(_) => 0,
            Command::Down(_) => 0,
            Command::Left(_) => -1,
            Command::Right(_) => 1,
        }
    }

    fn get_y_offset(self: &Command) -> i32 {
        return match self {
            Command::Up(_) => 1,
            Command::Down(_) => -1,
            Command::Left(_) => 0,
            Command::Right(_) => 0,
        }
    }

    fn print(self: &Command) {
        match self {
            Command::Up(x) => println!("Up {}", x),
            Command::Down(x) => println!("Down {}", x),
            Command::Left(x) => println!("Left {}", x),
            Command::Right(x) => println!("Right {}", x),
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
            "U" => Command::Up(parts[1].parse().unwrap()),
            "D" => Command::Down(parts[1].parse().unwrap()),
            "L" => Command::Left(parts[1].parse().unwrap()),
            "R" => Command::Right(parts[1].parse().unwrap()),
            _ => panic!("Unexpected command")
        };
        
        commands.push(command);
    }

    return Ok(commands)
}


fn solve_a(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let commands: Commands = parse_commands(input_string)?;
    
    let mut rope = Rope {
        head: Position{x: 0, y: 0},
        tail: Position{x: 0, y: 0}
    };
    let mut tail_positions : HashSet<String> = HashSet::new();

    for command in commands {
        let x_offset = command.get_x_offset();
        let y_offset = command.get_y_offset();
        command.print();
        for _ in 0..command.get_steps() {
            rope.move_head(x_offset, y_offset);
            rope.move_tail();

            let mut head_pos_string = "".to_string();
            rope.head.get_as_string(&mut head_pos_string);
            println!("Head -> {}", head_pos_string);


            let mut position_string = "".to_string();
            rope.tail.get_as_string(&mut position_string);
            println!("Tail -> {}", position_string);
            tail_positions.insert(position_string);

        }
    }

    return Ok(tail_positions.len() as i32);
}

fn solve_b(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let commands: Commands = parse_commands(input_string)?;

    let mut rope = LongRope {
        head: Position{x: 0, y: 0},
        tails: Vec::new()
    };

    for _ in 0..9 {
        rope.tails.push(Position{x: 0, y: 0})
    }

    let mut tail_positions : HashSet<String> = HashSet::new();

    for command in commands {
        let x_offset = command.get_x_offset();
        let y_offset = command.get_y_offset();
        command.print();
        for _ in 0..command.get_steps() {
            rope.move_head(x_offset, y_offset);
            rope.move_tails();

            let mut head_pos_string = "".to_string();
            rope.head.get_as_string(&mut head_pos_string);
            println!("Head -> {}", head_pos_string);


            let mut position_string = "".to_string();
            rope.tails[8].get_as_string(&mut position_string);
            println!("Tail -> {}", position_string);
            tail_positions.insert(position_string);

        }
    }

    return Ok(tail_positions.len() as i32);
}



fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
