use std::fs;
use std::collections::HashMap;
//use regex::{Captures, Regex, RegexSet};



pub fn main(){
    //let filename = "./inputs/day14_test.txt";
    let filename = "./inputs/day14.txt";
    
    let (polymer, rules) = process_file(filename);
    part_two( &polymer, &rules,10 );
    part_two( &polymer, &rules,40 );
}

//naive implementation that actually modifies the string
//only works up to n_iter = 20 in a reasonable amount of time...
//will not run because I changed rules to a hashmap in the improved version
fn part_one( polymer: &str, rules: &Vec<Vec<String>> ){

    let mut my_polymer = polymer.to_string();
    let n_iter = 10;
    for k in 0..n_iter{
        //first find the match locations
        let mut matches:Vec<usize> = Vec::new();
        let mut match_vals:Vec<String> = Vec::new();
        for i in 0..my_polymer.len()-1{
            for r in rules{
                if r[0] == my_polymer.get(i..i+2).unwrap(){
                    matches.push(i+1);
                    match_vals.push(r[1].to_string());
                    break;
                }
            }
        }

        //for each match, make the changes, but add 1 to all subsequent indices
        let mut new_string = String::from(my_polymer) ;
        let mut n_matches = 0;
        for i in 0..matches.len(){
            new_string.insert_str( matches[i]+n_matches, &match_vals[i] );
            n_matches += 1;
        }

        println!("{}", new_string.len());
        my_polymer = new_string.clone();

    }

    //count up the chars
    let mut counter:HashMap<String, usize> = HashMap::new();
    let temp: Vec<&str> = my_polymer.split("").collect();
    
    for c in temp[1..temp.len()-1].into_iter(){
        if !counter.contains_key( &c.to_string() ){
            counter.insert(c.to_string(),  1  );
        }else{
            let x = counter.get_mut( &c.to_string()  ).unwrap();
            *x += 1;
        }
    }
    
    let mut max_val = 0;
    let mut min_val = 0;
    for (key,value) in counter.iter(){
        if *value > max_val{
            max_val = *value;
        }else if *value < min_val || min_val == 0{
            min_val = *value;
        }
    }
    println!("Part one: {}", max_val-min_val );
    println!("{:?}", my_polymer);
}


//efficient implementation!
//don't keep the string, keep the occurences. It makes sooo much more sense
fn part_two(polymer: &str, rules: &HashMap<String, String>, n_iter:usize ){

    let mut pair_cts: HashMap<String, usize> = HashMap::new();
    let mut single_cts: HashMap<String, usize> = HashMap::new();

    for i in 0..polymer.len()-1{
        for (k,_v) in rules{
            if k == polymer.get(i..i+2).unwrap(){
                if !pair_cts.contains_key( &polymer.get(i..i+2).unwrap().to_string()  ){
                    pair_cts.insert(polymer.get(i..i+2).unwrap().to_string(),  1  );
                }else{
                    let x = pair_cts.get_mut( &polymer.get(i..i+2).unwrap().to_string()  ).unwrap();
                    *x += 1;
                }
                break;
            }
        }
    }
    let temp: Vec<&str> = polymer.split("").collect();
    for c in temp[1..temp.len()-1].into_iter(){
        if !single_cts.contains_key( &c.to_string() ){
            single_cts.insert(c.to_string(),  1  );
        }else{
            let x = single_cts.get_mut( &c.to_string()  ).unwrap();
            *x += 1;
        }
    }

    for _k in 0..n_iter{

        for (pair, count) in pair_cts.clone(){
            let x = pair_cts.get_mut( &pair  ).unwrap();
            *x -= count;

            let new_pair_one = String::from_iter([pair.get(..1).unwrap(), rules.get(&pair).unwrap()    ]);
            let new_pair_two = String::from_iter([rules.get(&pair).unwrap(), pair.get(1..).unwrap()    ]);
            if !pair_cts.contains_key( &new_pair_one ){
                pair_cts.insert(new_pair_one,  count  );
            }else{
                let x = pair_cts.get_mut( &new_pair_one  ).unwrap();
                *x += count;
            }

            if !pair_cts.contains_key( &new_pair_two ){
                pair_cts.insert(new_pair_two,  count  );
            }else{
                let x = pair_cts.get_mut( &new_pair_two  ).unwrap();
                *x += count;
            }

            //println!("{}",pair);
            if single_cts.contains_key(rules.get(&pair).unwrap()){
                let x = single_cts.get_mut( rules.get(&pair).unwrap()  ).unwrap();
                *x += count;
            }else{
                single_cts.insert((&rules.get(&pair).unwrap()).to_string(), count );
            }
        }
    }

    
    let mut max_val = 0;
    let mut min_val = 0;
    for (key,value) in single_cts.iter(){
        if *value > max_val{
            max_val = *value;
        }else if *value < min_val || min_val == 0{
            min_val = *value;
        }
    }
    println!("Part one/two improved: After {} iterations {}",n_iter, max_val-min_val );


}


//read the input data, 
fn process_file(filename: &str) -> (String, HashMap<String, String>){

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let polymer: String = lines[0].trim().to_string();

    let mut v : HashMap<String, String> = HashMap::new();
    for i in 2..lines.len(){
        let l = lines[i];
        let temp:Vec<&str> = l.trim().split("->").collect();
        //let x:Vec<usize> = temp[1..temp.len()-1].into_iter().map( |x| x.trim().parse::<usize>().unwrap()  ).collect();
        //v.push(x);
        let pair:String = temp[0].trim().to_string();
        let insertion:String = temp[1].trim().to_string();
        v.insert( pair, insertion  );
    }
    (polymer, v)
}