
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
}