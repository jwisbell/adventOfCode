use std::fs;
use std::io;

pub fn read_lines(filename: &str)-> String{
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    lines
}