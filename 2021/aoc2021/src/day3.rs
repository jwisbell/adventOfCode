use std::fs;

//test input had 5 bits, real has 12
const BITS:usize = 12;


pub fn main(){
    //let filename = "./inputs/day3a_test.txt";
    let filename = "./inputs/day3a.txt";
    let commands = process_file(filename);
    
    part_one(&commands);
    part_two(&commands);
}

fn part_one(v: &Vec<[usize;BITS]>){
    //sum each column 
    let sum = summation(v);
    let mut gamma = [0;BITS];
    let mut epsilon = [0;BITS];
    for i in 0..sum.len(){
        //assign the bit value based on what was the majority
        let bit = sum[i]>v.len()/2;
        gamma[i] = bit as usize;
        epsilon[i] = !bit as usize; //opposite of gamma
    }
    //convert to strings and then convert to decimal
    let mut gamma_s = String::from("");
    let mut epsilon_s = String::from("");
    //push each bit on
    for i in 0..sum.len(){
        gamma_s.push_str( &format!("{}",gamma[i])  );
        epsilon_s.push_str( &format!("{}",epsilon[i])  );
    }
    //convert to decimal
    let gamma = usize::from_str_radix(&gamma_s, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_s, 2).unwrap();
    println!("Part one: {}x{}={}", gamma, epsilon, gamma*epsilon);
}

fn part_two(v: &Vec<[usize;BITS]>){
    let ox = gas(&v, "oxygen");
    let co2 = gas(&v, "co2");
    println!("Part two: {}x{}={}",ox, co2, ox*co2);
}

fn gas(v: &Vec<[usize;BITS]>, species:&str ) -> usize{
    let mut sum = summation(v);
    let mut valid = v.clone();
    //find most/least common value in bit (left->right), discard others
    //stop when one number remains
    for i in 0..sum.len(){
        if valid.len() == 1{
            break;
        }
        //for oxygen keep most common value
        let mut bit = (sum[i] as f32 >= (valid.len() as f32) /2.0) ;
        if species == "co2"{
            bit = !bit; //opposite for co2
        }
        let bit = bit as usize; //convert to int for bits
        
        //iterate over remaining valid options and keep those with correct value
        //in bit i
        let mut new_valid:Vec<[usize;BITS]> =  Vec::new();
        for j in 0..valid.len(){
            if valid[j][i] == bit   {
                new_valid.push( valid[j]  );
            }
        }
        valid = new_valid;
        sum = summation(&valid); //recalculate the most/least common value in each column
        //could be cheaper, instead of doing addition for all bits, just do the relevant one
    }
    println!("{} {:?}",species, valid);
    //convert to string and then to decimal
    let mut valid_s = String::from("");
    for i in 0..sum.len(){
        valid_s.push_str( &format!("{}",valid[0][i])  );
    }
    usize::from_str_radix(&valid_s, 2).unwrap()
}

//add values at each index together
fn add(x:[usize;BITS],y:[usize;BITS]) -> [usize;BITS]{
    let mut z:[usize;BITS] = [0;BITS];
    for i in 0..BITS{
        z[i] = x[i] + y[i];
    }
    z
}

//calculate the sum in each bit column
fn summation(v: &Vec<[usize;BITS]> ) -> [usize;BITS]{
    let mut sum: [usize;BITS] = [0;BITS];
    for x in v{
        sum = add(sum, *x);
    }
    sum
}


//modified from yesterday to return the vec of int arrays
fn process_file(filename: &str) -> Vec<[usize;BITS] >{
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    //each line should be formatted as
    // 00100
    //etc.
    let mut v : Vec<[usize;BITS]> = Vec::new();
    for l in lines{
        //split the line into bits
        let temp:Vec<&str> = l.split(" ").collect();
        let mut a:[usize;BITS] = [0;BITS];
        for i in 0..BITS{
            
            a[i] = temp[0][i..i+1].trim().parse::<usize>().unwrap(); 
        }
        v.push(a);
    }
    v
}