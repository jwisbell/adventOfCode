use std::fs;

pub fn read_lines(filename: &str) -> Vec<String> {
    //this part is the same for all files...
    println!("Processing {}", filename);
    let contents = fs::read_to_string(&filename).expect("Something went wrong reading the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();
    let _ = lines.pop();

    let mut values: Vec<String> = Vec::new();
    for l in lines {
        values.push(String::from(l))
    }
    values
}
