extern crate core;

use std::cmp::max;
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

type Trees = Vec<Vec<Tree>>;

struct Tree {
    height: u32,
    visible: bool
}

struct Forest {
    trees: Trees,
    rows: i32,
    columns: i32
}

impl Forest {
    fn mark_outer_visible(self: &mut Forest) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                if row == 0 || column == 0 || row == self.rows - 1 || column == self.columns - 1 {
                    self.trees[row as usize][column as usize].visible = true
                }
            }
        }
    }

    fn mark_inner_visible(self: &mut Forest) {
        // Consider visibilty from the left
        for row in 0..self.rows {
            let mut max_height = 0;
            for column in 0..self.columns {
                let ref mut tree: Tree = self.trees[row as usize][column as usize];
                if max_height < tree.height {
                    tree.visible = true;
                }
                max_height = max(max_height, tree.height)
            }
        }

        // Consider visibility from the right
        for row in 0..self.rows {
            let mut max_height = 0;
            for column_offset in 0..self.columns {
                let column = self.columns - 1 - column_offset;
                let ref mut tree: Tree = self.trees[row as usize][column as usize];
                if max_height < tree.height {
                    tree.visible = true;
                }
                max_height = max(max_height, tree.height)
            }
        }

        // Consider visibility from the top
        for column in 0..self.columns {
            let mut max_height = 0;
            for row in 0..self.rows {
                let ref mut tree: Tree = self.trees[row as usize][column as usize];
                if max_height < tree.height {
                    tree.visible = true;
                }
                max_height = max(max_height, tree.height)
            }
        }

        // Consider visibility from the bottom
        for column in 0..self.columns {
            let mut max_height = 0;
            for row_offset in 0..self.rows {
                let row = self.rows - 1 - row_offset;
                let ref mut tree: Tree = self.trees[row as usize][column as usize];
                if max_height < tree.height {
                    tree.visible = true;
                }
                max_height = max(max_height, tree.height)
            }
        }
    }

    fn get_score(self: &mut Forest, row: usize, column: usize) -> i32 {
        let ref tree: Tree = self.trees[row][column];
        let mut score = 1;

        // look left
        let mut left_score = 0;
        for column_offset in 1..column + 1 {
            let this_column = column - column_offset;
            let ref this_tree = self.trees[row][this_column];
            left_score += 1;
            if this_tree.height >= tree.height {
                // view blocked, pack it in
                break
            }
        }
        score = score * left_score;

        // look right
        let mut right_score = 0;
        for this_column in column + 1..self.columns as usize {
            let ref this_tree = self.trees[row][this_column];
            right_score += 1;
            if this_tree.height >= tree.height {
                // view blocked, pack it in
                break
            }
        }
        score = score * right_score;

        // look up
        let mut up_score = 0;
        for row_offset in 1..row + 1 {
            let this_row = row - row_offset;
            let ref this_tree = self.trees[this_row][column];
            up_score += 1;
            if this_tree.height >= tree.height {
                // view blocked, pack it in
                break
            }
        }
        score = score * up_score;

        // look down
        let mut down_score = 0;
        for this_row in row + 1..self.rows as usize {
            let ref this_tree = self.trees[this_row][column];
            down_score += 1;
            if this_tree.height >= tree.height {
                // view blocked, pack it in
                break
            }
        }
        score = score * down_score;

        return score;
    }

    fn count_visible(self: &mut Forest) -> i32 {
        let mut visible = 0;
        for row in 0..self.rows {
            for column in 0..self.columns {
                if self.trees[row as usize][column as usize].visible {
                    visible += 1
                }
            }
        }

        return visible;
    }

    fn print(self: &mut Forest) {
        for row in 0..self.rows {
            let mut row_string : String = "".to_string();
            for column in 0..self.columns {
                let ref tree = self.trees[row as usize][column as usize];
                row_string.push_str(&tree.height.to_string());
            }
            println!("{}", row_string);
        }
    }

    fn print_visible(self: &mut Forest) {
        for row in 0..self.rows {
            let mut row_string : String = "".to_string();
            for column in 0..self.columns {
                if self.trees[row as usize][column as usize].visible {
                    row_string.push('1');
                } else {
                    row_string.push('0');
                }
            }
            println!("{}", row_string);
        }
    }
}

fn parse_forest(input: String) -> ResultOrErr<Forest> {
    let mut trees : Trees = Vec::new();
    let mut rows = 0;

    let mut columns = 0;
    for line in input.lines() {
        if line == "" {
            continue;
        }

        let mut row: Vec<Tree> = Vec::new();
        rows += 1;

       columns = 0;
        for char in line.chars() {
            row.push(Tree{height: char.to_digit(10).unwrap(), visible: false});
            columns +=1;
        }

        trees.push(row);
    }

    return Ok(Forest{trees, rows, columns})
}


fn solve_a(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let mut forest: Forest = parse_forest(input_string)?;
    forest.mark_outer_visible();
    forest.mark_inner_visible();

    forest.print();
    println!("-------------------");
    forest.print_visible();

    return Ok(forest.count_visible());
}

fn solve_b(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let mut forest: Forest = parse_forest(input_string)?;

    let mut best_score = 0;
    for row in 0..forest.rows {
        for column in 0..forest.columns {
            best_score = max(best_score, forest.get_score(row as usize, column as usize))
        }
    }

    return Ok(best_score);
}



fn load_input(input_filename: &str) -> ResultOrErr<String> {
    return match fs::read_to_string(input_filename) {
        Ok(x) => Ok(x),
        Err(x) => Err(x.to_string())
    };
}
