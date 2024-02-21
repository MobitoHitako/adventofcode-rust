use std::{fs};
use std::io::{Read};

fn handle_floor_instructions(content: &String, first_basement_check: bool) -> i64{
    let mut current_floor = 0;
    let mut current_position = 0;
    for content in content.chars() {
        if current_floor == -1 && first_basement_check  {
            println!(" At Position {}, Santa Goes first into the Basement!", current_position);
            break;
        }
        match content{
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => ()
        };
        current_position += 1;

    }

    current_floor
}


pub fn run(first_basement_check: bool) {
    println!("Welcome to Day01");
    const FILE_PATH: &str = "src/adv2015/input/Day01/day01.txt";

    // Read Contents from File
    let contents = fs::read_to_string(FILE_PATH).expect("Cannot read File");
    let result = handle_floor_instructions(&contents, first_basement_check);


    println!("Santa ended up in Floor: {}", result)
}