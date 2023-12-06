use std::io;

//my modules
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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

        println!("Executing day {}",day);
        //match block to execute code for specific day
        match day{
            1 => {day1::main(); break;},
            2 => {day2::main(); break;},
            3 => {day3::main(); break;},
            4 => {day4::main(); break;},
            5 => {day5::main(); break;},
            6 => {day6::main(); break;},
            _=> println!("Please enter a valid day to run (0-24): "),//edge case, keep looping
        }
    }
}
