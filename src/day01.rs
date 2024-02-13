use std::io::Read;
use std::mem::swap;

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


    pub fn update_max_calories_array(vec: &mut Vec<Elves>){
        for i in 0..vec.len() {
            vec[i].update_max_calories();
        }
    }

}


fn sort_vec(vec: &mut Vec<Elves>){
    for i in 0..vec.len() {
        // println!("Index: {i}");
        //  println!("Vector Value: {}", vec.get(i).unwrap().max_calories);
        for j in 0..vec.len() - 1 {
            if vec[j].max_calories  > vec[j+1].max_calories {
                vec.swap(j, j+1);
            }
        }
    }
}


pub fn run() {
    println!("Welcome to Day 01 of Rust Advent of Code!");
   let elf01 = Elves::new("Elf01".to_string(), vec![1000,2000,3000]);
   let elf02 = Elves::new("Elf02".to_string(), vec![1000]);
   let elf03 = Elves::new("Elf03".to_string(), vec![1000,1000,5000]);
   let elf04 = Elves::new("Elf04".to_string(), vec![2000,3000]);

    let mut elves: Vec<Elves> = vec![ elf01, elf02, elf03, elf04];
    Elves::update_max_calories_array(&mut elves);

    sort_vec(&mut elves);

    for i in 0..elves.len() {
        println!("{}", elves[i].max_calories)
    }

}
