use std::io::Read;

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
        let file = std::fs::File::open(file_path);

        Vec::new()
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
}


pub fn run() {
    println!("Welcome to Day 01 of Rust Advent of Code!");
    let mut elv01 = Elves::new(String::from("Elf01"), vec![1000,2000,3000]);
    let mut elv02 = Elves::new(String::from("Elf02"), Vec::new());
    println!("{:?}", elv01.calories.iter_mut().collect::<Vec<_>>());
    elv01.calories.push(2000);
    elv01.calories.push(3000);
    elv01.calories.push(4000);

    println!("should be empty {:?}", elv02.calories.iter_mut().collect::<Vec<_>>());
    println!("{}", elv01.update_max_calories());

    println!("{:?}", elv01.max_calories);
    elv01.max_calories = 2;
    println!("{:?}", elv01.max_calories);



}
