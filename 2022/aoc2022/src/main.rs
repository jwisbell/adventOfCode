#![allow(dead_code)]
use std::io;
//my modules
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
            _=> println!("Please enter a valid day to run (0-24): "),//edge case, keep looping
        }
    }
}
