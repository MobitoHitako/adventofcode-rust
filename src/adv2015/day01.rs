use std::{fs};
use std::io::{Read};

fn handle_floor_instructions(content: &String) -> i64{
    let mut current_floor = 0;

    for content in content.chars() {
        match content{
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => ()
        };
    }

    current_floor
}


pub fn run() {
    println!("Welcome to Day01");
    let mut result = 0;
    const FILE_PATH: &str = "src/adv2015/input/Day01/day01.txt";
    // Open the file in read-only mode
    let contents = fs::read_to_string(FILE_PATH).expect("Cannot read File");

    result = handle_floor_instructions(&contents);


    println!("Santa ended up in Floor: {}", result)
}