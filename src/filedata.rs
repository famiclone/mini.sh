use std::fs::{metadata, read_to_string, write};

fn read_file(filename: &str) -> String {
    read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn write_file(filename: &str, contents: String) {
    write(filename, contents).expect("Unable to write file");
}

pub struct FileData {
    pub contents: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub path: String,
}

impl FileData {
    pub fn new(filepath: &str) -> Self {
        Self {
            contents: read_file(filepath),
            size: metadata(filepath).unwrap().len(),
            name: filepath.split(".").next().unwrap().to_string(),
            extension: filepath.split(".").last().unwrap().to_string(),
            path: filepath.to_string(),
        }
    }
}
