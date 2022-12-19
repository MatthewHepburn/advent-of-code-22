extern crate core;

use std::env;
use std::fs;
use std::cmp::min;

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

type Height = u8;

struct MapLocation {
    height: Height,
    distance: Option<i32>,
    is_start: bool,
    is_end: bool
}

impl MapLocation {
    fn char_to_height(char: char) -> Height {
        ((char as u8) - ('a' as u8)).try_into().unwrap()
    }
    fn height_to_char(height: Height) -> char {
        (height + ('a' as u8)) as char
    }
    fn from_char(char: char) -> MapLocation {
        if char == 'S' {
            MapLocation{
                height: MapLocation::char_to_height('a'),
                distance: Some(0),
                is_start: true,
                is_end: false
            }
        } else if char == 'E' {
            MapLocation{
                height: MapLocation::char_to_height('z'),
                distance: None,
                is_end: true,
                is_start: false
            }
        } else {
            MapLocation{
                height: MapLocation::char_to_height(char),
                distance: None,
                is_start: false,
                is_end: false
            }
        }
    }
}

struct Map {
    locations: Vec<Vec<MapLocation>>,
    rows: usize,
    columns: usize
}

impl Map {
    fn print_heights(self: &Map) {
        for row in &self.locations {
            for location in row {
                let char = if location.is_start {
                    'S'
                } else if location.is_end {
                    'E'
                } else {
                    MapLocation::height_to_char(location.height)
                };
                print!("{}", char);
            }
            println!();
        }
    }

    fn print_distances(self: &Map) {
        for row in &self.locations {
            for location in row {
                let char = match location.distance {
                    Some(x) => ((x as u8) + ('a' as u8)) as char,
                    None => '?'
                };
                print!("{}", char);
            }
            println!();
        }
    }

    fn get_neighbours(self: &Map, row: usize, column: usize) -> Vec<(usize, usize)> {
        let mut output: Vec<(usize, usize)> = Vec::new();
        if row > 0 {
            output.push((row - 1, column));
        }
        if column > 0 {
            output.push((row, column - 1));
        }
        if row < self.rows - 1 {
            output.push((row + 1, column));
        }
        if column < self.columns - 1 {
            output.push((row, column + 1));
        }

        return output;
    }
}

fn parse_map(input: String) -> ResultOrErr<Map> {
    let mut map = Map {
        locations: Vec::new(),
        rows: 0,
        columns: 0
    };

    for line in input.lines() {
        if line == "" {
            continue;
        }

        let mut row: Vec<MapLocation> = Vec::new();

        let chars : Vec<char> = line.chars().collect();
        for char in chars {
            let location = MapLocation::from_char(char);
            row.push(location);
        }

        map.columns = row.len();
        map.rows += 1;
        map.locations.push(row);
    }

    return Ok(map)
}

fn solve_a(input_filename: &str) -> ResultOrErr<i32> {
    let input_string = load_input(input_filename)?;
    let mut map: Map = parse_map(input_string)?;

    map.print_heights();
    println!("----------");
    map.print_distances();
    println!("----------");

    let mut changed = true;
    while changed {
        changed = false;
        for row_index in 0..map.rows {
            for column_index in 0..map.columns {
                if map.locations[row_index][column_index].distance == None {
                    println!("Skipping {},{} - no distance to here", row_index, column_index);
                    continue;
                }
                let this_distance = map.locations[row_index][column_index].distance.unwrap();
                let this_height = map.locations[row_index][column_index].height;
                for (neighbour_row, neighbour_column) in map.get_neighbours(row_index, column_index) {
                    // println!("Map has {} rows, {} cols. Considering neighbour {}, {}", map.rows, map.columns, neighbour_row, neighbour_column);
                    let neighbour_height = map.locations[neighbour_row][neighbour_column].height;
                    if neighbour_height > this_height + 1 {
                        println!("Skipping {},{} - too high", neighbour_row, neighbour_column);
                        // Neighbour too high - can't go this way
                        continue;
                    }

                    let neighbour_distance = map.locations[neighbour_row][neighbour_column].distance;
                    let new_neighbour_distance = match neighbour_distance {
                        None => this_distance + 1,
                        Some(x) => min(x, this_distance + 1)
                    };
                    if neighbour_distance != Some(new_neighbour_distance) {
                        changed = true;
                    }
                    map.locations[neighbour_row][neighbour_column].distance = Some(new_neighbour_distance)
                }
            }
        }

        println!("----------");
        map.print_distances();
    }


    println!("----------");
    map.print_distances();

    for row_index in 0..map.rows {
        for column_index in 0..map.columns {
            if map.locations[row_index][column_index].is_end {
                return match map.locations[row_index][column_index].distance {
                    Some(x) => Ok(x),
                    None => Err("Ended with no path to end".to_string())
                }
            }
        }
    }

    return Err("Could not find end location".to_string())
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
