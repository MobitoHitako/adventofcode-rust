use std::{fs};
use std::io::{Read};

fn handle_floor_instructions(content: &String, check_part_2: bool) -> i64{
    let mut current_floor = 0;
    let mut current_position = 0;
    for content in content.chars() {
        if current_floor == -1 && check_part_2 {
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


pub fn run(check_part_2: bool) {
    println!("Welcome to Day01");
    const FILE_PATH: &str = "src/adv2015/input/Day01/day01.txt";

    // Read Contents from File
    let contents = fs::read_to_string(FILE_PATH).expect("Cannot read File");
    let result = handle_floor_instructions(&contents, check_part_2);


    println!("Santa ended up in Floor: {}", result)
}