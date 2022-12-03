#![allow(dead_code)]
use std::io;
//my modules
mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;


fn main(){
    //This function simply calls the Function of the Day

    //For ease of use, I'll build it as a 'match' check 
    //so only one variable needs to be changed.
    //We'll get that from std input
    println!("Please enter a day to run (0-24): ");
    
    //loop until a valid day is entered
    loop{
        //input string
        let mut input_date = String::new();
        io::stdin()
            .read_line(&mut input_date)
            .expect("Failed to read line");
        
        //parse the number, in case of error set to edge case
        let day: u8 = match input_date.trim().parse(){
            Ok(num) => num,
            Err(_) => 255,
        };

        
        //match block to execute code for specific day
        match day{
                0 => {  println!("Executing day {}",day);
                        day0::main(); 
                        break;},
                1 => {  println!("Executing day {}",day);
                        day1::main(); 
                        break;},
                2 => {  println!("Executing day {}",day);
                        day2::main(); 
                        break;},
                3 => {  println!("Executing day {}",day);
                        day3::main(); 
                        break;},
                4 => {  println!("Executing day {}",day);
                        day4::main(); 
                        break;},
                5 => {  println!("Executing day {}",day);
                        day5::main(); 
                        break;},
                6 => {  println!("Executing day {}",day);
                        day6::main(); 
                        break;},
                7 => {  println!("Executing day {}",day);
                        day7::main(); 
                        break;},
                8 => {  println!("Executing day {}",day);
                        day8::main(); 
                        break;},
                9 => {  println!("Executing day {}",day);
                        day9::main(); 
                        break;},
                10 => {  println!("Executing day {}",day);
                        day10::main(); 
                        break;},
                11 => {  println!("Executing day {}",day);
                        day11::main(); 
                        break;},
                12 => {  println!("Executing day {}",day);
                        day12::main(); 
                        break;},
                13 => {  println!("Executing day {}",day);
                        day13::main(); 
                        break;},
                14 => {  println!("Executing day {}",day);
                        day14::main(); 
                        break;},
                15 => {  println!("Executing day {}",day);
                        day15::main(); 
                        break;},
                16 => {  println!("Executing day {}",day);
                        day16::main(); 
                        break;},
            _=> println!("Please enter a valid day to run (0-24): "),//edge case, keep looping
        }
    }
    
    
    
    
    



}


