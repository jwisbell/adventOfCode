use std::fs;
use std::io;

pub fn main(){
    println!("Enter input file");
    let mut filename = String::new();
        io::stdin()
            .read_line(&mut filename)
            .expect("Failed to read line");
    filename = filename.trim().to_string();
    let depths = process_file( &filename );

    part_one(&depths);
    part_two(&depths);
}


fn part_one(v: &Vec<isize>){    
    //now compute the relative depths
    //make a new vector to save values computed from naive looping
    let mut offsets:Vec<isize> = Vec::new();
    offsets.push(0);
    let mut n_increased = 0;
    for i in 1..v.len(){
        //println!("{}", v[i]);
        offsets.push(v[i]- v[i-1] );
        if v[i]- v[i-1] > 0{
            n_increased = n_increased + 1;
        }
    }
    println!("Part 1 answer: {}",n_increased);
}

fn part_two(v: &Vec<isize>){
    //now compute the relative depths of a sliding window
    //make a new vector to save values computed from naive looping
    let mut offsets:Vec<isize> = Vec::new();
    offsets.push(0);
    let mut n_increased = 0;
    for i in 1..v.len()-2{
        let window1_val = v[i-1] + v[i] + v[i+1];
        let window2_val = v[i] + v[i+1] + v[i+2];

        offsets.push(window2_val - window1_val );
        if window2_val - window1_val > 0{
            n_increased = n_increased + 1;
        }
    }
    println!("Part two answer: {}",n_increased);
}


fn process_file(filename: &str) -> Vec<isize>{
    println!("Processing {}",filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split_whitespace().collect();
    let mut v : Vec<isize> = Vec::new();
    for l in lines{
        v.push(  l.parse::<isize>().unwrap()  );
    }
    v
}