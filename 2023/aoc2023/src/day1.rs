use std::fs;
use regex::{Regex,RegexSet};

pub fn main(){
    let filename = "./inputs/day1_test.txt";
    let filename = "./inputs/day1.txt";
    let lines = process_input(filename);

    part1(lines.clone());
    part2(lines);
}

fn part1(lines:Vec<String>){
    println!("Part 1");
    let re = Regex::new(r"([0-9])").unwrap();
    let mut results: Vec<u64> = Vec::new();
    for l in lines{
        //first identify the numbers
        let numbers: Vec<&str> = re.find_iter(&l).map(|m| m.as_str()).collect();
        
        //then take first and last
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        
        results.push( format!("{}{}",first,last).parse::<u64>().unwrap() )
    }
    println!("Individual values {:?}", results);
    let sum: u64 = results.iter().sum();
    println!("Sum {:?}", sum );
}


fn regex_replacer(hay:&str) -> Vec<&str>{
    //i want to find all occurrences of a set of substrings representing integers
    //return a string consisting of integers in the order they appear
    let searches = ["1","2","3","4","5","6","7","8","9","0",
        "one","two","three","four","five","six","seven","eight","nine","ten",
        "eleven","twelve","thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","nineteen"];
    
    //make an empty array with length same as the maximum line length
    let mut mystrvec: Vec<&str> = vec![""; 100];
    for s in searches{
        let matches: Vec<_> = hay.match_indices(s).collect();
        //println!("{} is found at {:?}",s, matches);
        for m in matches{
            mystrvec[m.0] = string_to_num(&s);//.clone().to_string().as_str();
        }
    }
    
    return mystrvec.clone()
}

fn part2(lines:Vec<String>){
    println!("Part 2");
    //it turns out overlapping regex in rust isn't possible with standard packages
    //i just search for all substrings and build up the final string from those matches
    
    let re = Regex::new(r"([0-9])").unwrap();
    let mut results: Vec<u64> = Vec::new();
    for l in lines{
        
        //create the vector of matched substrings 1-9 and one-nineteen
        let mystringvec =  regex_replacer(&l);
        let mystring = String::from_iter(mystringvec);
        
        //identify the numbers with regex
        let numbers: Vec<&str> = re.find_iter(&mystring).map(|m| m.as_str()).collect();
        
        //then take first and last
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        
        results.push( format!("{}{}",first,last).parse::<u64>().unwrap() );
        //println!("{}, {:?}, {:?}", l.trim(), mystring, format!("{}{}",first,last));
    }
    println!("Individual values {:?}", results);
    let sum: u64 = results.iter().sum();
    println!("Sum {:?}", sum );
    
}

fn string_to_num(x:&str)->&str{
    //fn to convert string number names into string integers
    match x {
        "one"=>"1",
        "two"=>"2",
        "three"=>"3",
        "four"=>"4",
        "five"=>"5",
        "six"=>"6",
        "seven"=>"7",
        "eight"=>"8",
        "nine"=>"9",
        "ten"=>"10",
        "eleven"=>"11",
        "twelve"=>"12",
        "thirteen"=>"13",
        "fourteen"=>"14",
        "fifteen"=>"15",
        "sixteen"=>"16",
        "seventeen"=>"17",
        "eighteen"=>"18",
        "nineteen"=>"19",
        "twenty"=>"20",
        _=>x
    }
}

fn process_input(filename:&str)->Vec<String>{
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut calibrations: Vec<String> = Vec::new();
    for l in lines{
        calibrations.push( String::from(l) )
    }
    calibrations
}