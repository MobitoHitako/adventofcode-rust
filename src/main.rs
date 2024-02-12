mod day01;

fn main() {
    let mut elf01 = day01::Elves::new("Elf01".to_string(), vec![1000,2000]);
    println!("{:?}", elf01.get_max_calories());

    let mut elves = day01::Elves::new_by_file("src/day01/testing.txt".to_string());
    println!("{:?}", elves);
}



