
// Declaring my Libary
pub mod mobi_library{
    use std::fs::File;
    use std::io::BufReader;

    // Declaring public Function named read_file
    pub fn read_file(file_path: &str) -> BufReader<File>{
        println!("reading File from: {}", file_path);
        let file = File::open(file_path)
            .expect("Cannot Open File Try Again");

        // Create a BufReader to efficiently read lines
        BufReader::new(file)
    }

   pub fn sort_vec(vec: &mut Vec<i32>){
        for _i in 0..vec.len() {
            for j in 0..vec.len() - 1 {
                if vec[j]  > vec[j+1] {
                    vec.swap(j, j+1);
                }
            }
        }
    }
}