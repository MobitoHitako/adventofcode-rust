use std::fs;
use advent_of_code::mobi_library::sort_vec;


///
fn part1_calculate_surface_area(l: &i32, w: &i32, h: &i32) -> (i32){

    let side_l_w = l * w;
    let side_w_h = w * h;
    let side_h_l = h * l;

    let surface_area_box=(2*side_l_w) + (2*side_w_h) + (2*side_h_l);
    let smallest_side_area = side_l_w.min(side_w_h).min(side_h_l);

     ( surface_area_box + smallest_side_area)
}


fn part2(l: &i32, w: &i32, h: &i32) -> i32 {
    let mut temp_vec = vec![*l,*h,*w];
    sort_vec(&mut temp_vec);
    // println!("first_smallest: {} , second smallest: {}",temp_vec[0], temp_vec[1]);

    let a = l * w * h;
    let b = 2*temp_vec[0]+2*temp_vec[1];
    a+b
}



fn handle_file_contents(content: &str, check_part_2: bool) -> i32{
    if !content.contains("x") {
        println!("cannot Handle this one sry");
        return 0;
    }

    let split_terminator = "x";
    let content_split: Vec<&str> = content.split_terminator(split_terminator).collect();

    if !content_split.len() == 3 {
        println!("content requires a length, height and width!");
        return 0;
    }

    let l:i32 =  content_split[0].parse().unwrap();
    let w:i32 =  content_split[1].parse().unwrap();
    let h:i32 =  content_split[2].parse().unwrap();
    println!("{:?}", content_split);


    return if check_part_2 {
        part2(&l, &w, &h)
    } else {
        part1_calculate_surface_area(&l, &w, &h)
    }

}

pub fn run(check_part_2: bool){
 println!("Welcome to Day02");
    //Length        Width       Height
    // 2      x     3     x     4
    const FILE_PATH: &str = "src/adv2015/input/Day02/day02.txt";
    let (contents)= fs::read_to_string(FILE_PATH)
        .expect("Could not read File");

    let mut total_wrapping_paper = 0;

    for content in contents.lines(){
        total_wrapping_paper += handle_file_contents(content, check_part_2);
    }

    println!("{}", total_wrapping_paper);
}