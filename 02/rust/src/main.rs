use std::env;
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

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

struct Game {
    my_move: Move,
    opponent_move: Move
}


struct GamePlan {
    outcome: Outcome,
    opponent_move: Move
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose
}

fn score_game(game: Game) -> i32 {
    let move_score = match game.my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    };

    if game.my_move == game.opponent_move {
        // Draw!
        return move_score + 3;
    }

    // We know we have a winner - was it us?
    if game.my_move == Move::Rock && game.opponent_move == Move::Scissors {
        return move_score + 6
    }
    if game.my_move == Move::Paper && game.opponent_move == Move::Rock {
        return move_score + 6
    }
    if game.my_move == Move::Scissors && game.opponent_move == Move::Paper {
        return move_score + 6
    }

    // We lost!
    return move_score;
}

fn solve_a() -> i32 {
    let games = load_input_as_games();

    let mut my_score = 0;

    for game in games {
       my_score = my_score + score_game(game);
    }

    return my_score;
}

fn solve_b() -> i32 {
    let game_plans = load_input_as_game_plans();

    let mut my_score = 0;

    for game_plan in game_plans {
        if game_plan.outcome == Outcome::Draw {
            let game = Game{opponent_move: game_plan.opponent_move, my_move: game_plan.opponent_move};
            my_score = my_score + score_game(game);
        } else if game_plan.outcome == Outcome::Win {
            let my_move = match game_plan.opponent_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock
            };
            let game = Game{opponent_move: game_plan.opponent_move, my_move};
            my_score = my_score + score_game(game);
        } else {
            let my_move = match game_plan.opponent_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper
            };
            let game = Game{opponent_move: game_plan.opponent_move, my_move};
            my_score = my_score + score_game(game);
        }
    }

    return my_score;
}

fn is_example_mode() -> bool {
    let example_mode = env::var("AOC_EXAMPLE_MODE");
    if example_mode.is_err() {
        return false;
    }

    return example_mode.unwrap() == "1"
}

fn load_input_as_games() -> Vec<Game> {
    let filename = if is_example_mode() { "exampleInput.txt" } else { "input.txt" };
    let mut path = String::from("../");
    path.push_str(filename);

    let contents : String = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let mut output: Vec<Game> = Vec::new();
    for line in contents.lines() {
        if line == "" {
            continue
        }
        let opponent_move: Move = match line.chars().nth(0).unwrap() {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("Could not parse opponent_move")
        };
        let my_move: Move = match line.chars().nth(2).unwrap() {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("Could not parse my_move")
        };
        output.push(Game{my_move, opponent_move});
    }

    return output;
}

fn load_input_as_game_plans() -> Vec<GamePlan> {
    let filename = if is_example_mode() { "exampleInput.txt" } else { "input.txt" };
    let mut path = String::from("../");
    path.push_str(filename);

    let contents : String = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let mut output: Vec<GamePlan> = Vec::new();
    for line in contents.lines() {
        if line == "" {
            continue
        }
        let opponent_move: Move = match line.chars().nth(0).unwrap() {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("Could not parse opponent_move")
        };
        let outcome: Outcome = match line.chars().nth(2).unwrap() {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Could not parse outcome")
        };
        output.push(GamePlan{opponent_move, outcome});
    }

    return output;
}
