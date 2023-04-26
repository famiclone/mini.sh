mod lexer;
mod filedata;
mod token;

//use lexer::Lexer;
//use token::Token;
use std::fs::metadata;
pub use filedata::FileData;

use crate::filedata::write_file;

const PREFIX: &str = ".min.";

enum TokenType {
    Comment,
    Function,
    Const,
    Variable,
    Identifier,
    Number,
    String,
    OParen,
    CParen,
    OCurl,
    CCurl,
    Return,
    SemiColon,
    FunctionCall,
    Equals
}

fn main() {
    let file = FileData::new("index.js");
    let new_filename = &(file.name.to_owned() + PREFIX + &file.extension);

    if !metadata(&new_filename).is_ok() {
        write_file(new_filename, String::new());
    }

    let new_file = FileData::new(new_filename);

    let mut last_sequence = String::new();
    let mut block_start: usize = 0;

    for (i, char) in file.contents.chars().enumerate() {
        last_sequence.push(char);
        
        if last_sequence == " " || last_sequence == "\n" || last_sequence == "\t" {
            last_sequence = String::new();
        }

        if last_sequence == "/**" {
            block_start = i - 2;
            last_sequence = String::new();
        }

        if last_sequence == "*/" {
            let block = &file.contents[block_start..i + 1];
            println!("Block: {}", block);
            //new_file.contents = new_file.contents.replace(block, "");
            last_sequence = String::new();
        }
    }

    write_file(&(new_file.path), file.contents.clone());

    println!("Input file: {}", file.name);
    println!("Size: {}", file.size);
    println!("-------------------");
    println!("Output file: {}", new_file.name);
    println!("Size: {}", new_file.size);
}
