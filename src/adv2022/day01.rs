use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Debug)]
pub(crate) struct Elves {
    name: String,
    calories: Vec<u32>,
    max_calories: u32
}

impl Elves {
     pub fn new(name: String, calories: Vec<u32>) -> Self {
        Self{
            name,
            calories,
            max_calories: 0,
        }
    }


    pub fn new_by_file(file_path: String) -> Vec<Elves>{
        let mut elves:Vec<Elves> = Vec::new();

        let file = File::open(file_path).expect("Cant open File!");
        let reader = BufReader::new(file);

        let mut calories:Vec<u32> = Vec::new();
        let mut count_elves = 1;
        //line by line
        for line in reader.lines(){
            let line = line.expect("Line could not be read");
            let parsed_line: u32;

           // PARSE STRING TO INTEGER, BUT ONLY IF IT CAN PARSE (im stupid)
           if line.parse::<u32>().ok().is_some() {
                parsed_line = line.parse().unwrap();
                calories.push(parsed_line);
            }

            // if line is empty that means we need to return the current Vector
            if line.is_empty()  {
                elves.push(Elves::new(String::from("Elf").add(&*count_elves.to_string()), calories.to_vec()));

                // After Push, we clear calories vector to make room for next Elf
                calories.clear();
                count_elves += 1;
            }

        }

        println!("Max Calories is being updated for you for using new_by_file()");
        Elves::update_max_calories_array(&mut elves);

        elves
    }

    pub fn get_max_calories(&self) -> u32 {
        if self.max_calories == 0 {
            println!("Try using update_max_calories() first!");
        }
        self.max_calories
    }
    
    pub fn update_max_calories(&mut self) -> u32{
    let mut max_calories = 0;

        if self.calories.is_empty() {
            return self.max_calories;
        }

        for i in 0..self.calories.len() {
            max_calories += self.calories[i];
        }
        self.max_calories = max_calories;

        return self.max_calories;
    }


    pub fn update_max_calories_array(vec: &mut Vec<Elves>){
        for i in 0..vec.len() {
            vec[i].update_max_calories();
        }
    }

}


fn sort_vec(vec: &mut Vec<Elves>){
    for _i in 0..vec.len() {
        // println!("Index: {i}");
        //  println!("Vector Value: {}", vec.get(i).unwrap().max_calories);
        for j in 0..vec.len() - 1 {
            if vec[j].max_calories  > vec[j+1].max_calories {
                vec.swap(j, j+1);
            }
        }
    }
}

fn find_last_three_max_calories(elves: &Vec<Elves>)-> u32{
    let mut sum: u32 = 0;
    for i in (elves.len() - 3..elves.len()).rev() {
        println!("{:?}", &elves[i]);
        sum += &elves[i].max_calories;
    }
    sum
}


pub fn run() {
    println!("Welcome to Day 01 of Rust Advent of Code!");
     let file_path: String = String::from("src/adv2022/input/Day01/advent01.txt");
    let mut elf_array = Elves::new_by_file(file_path);

    //Sort Vec
    sort_vec(&mut elf_array);

    // Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    let elf_most = &elf_array[elf_array.len() - 1];
    println!("The elf carrying the most is {} with the value of: {:?}", elf_most.name, elf_most.max_calories);

    //Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
    println!("Answer is: {}", find_last_three_max_calories(&elf_array));



}
