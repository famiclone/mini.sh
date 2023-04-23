use std::fs;
use std::fs::metadata;

const PREFIX: &str = ".min.";

struct FileData {
    contents: String,
    name: String,
    extension: String,
    size: u64,
}

impl FileData {
    fn new(filepath: &str) -> Self {
        let contents = read_file(filepath);
        let size = metadata(filepath).unwrap().len();
        let name = filepath.split(".").next().unwrap().to_string();
        let extension = filepath.split(".").last().unwrap().to_string();
        Self { contents, size, name, extension }
    }
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn write_file(filename: &str) {
    fs::write(filename, "hello world").expect("Unable to write file");
}

//struct Token {
//    // "keyword", "identifier", "string", "number", "operator", "punctuation"
//    token_type: String,
//    // e.g. "function", "foo", "bar", "1", "+", ";"
//    value: String,
//}
//
//impl Token {
//    fn new(token_type: String, value: String) -> Self {
//        Self { token_type, value }
//    }
//}

fn main() {
    let file = FileData::new("index.js");
    let new_file = FileData::new("index.min.js");

    if !metadata(&(file.name.to_owned() + PREFIX + &file.extension)).is_ok() {
        write_file(&(file.name.to_owned() + PREFIX + &file.extension));
    }
    file.contents;

    println!("Input file: {}", file.name);
    println!("Size: {}", file.size);
    println!("-------------------");
    println!("Output file: {}", new_file.name);
    println!("Size: {}", new_file.size);
}
