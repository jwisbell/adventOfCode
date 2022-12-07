use std::fs;
use std::collections::HashSet;


pub fn main(){
    //let filename = "./inputs/day3_test.txt";
    let filename = "./inputs/day3.txt";
    let rucksacks = process_input(filename);

    part_one(&rucksacks);
    part_two(&rucksacks);

}

fn part_one(rucksacks: &Vec<String>){
    //convert Vec<String> to Vec<&str>
    let rucksacks: Vec<&str> = rucksacks.iter().map(|s| &**s).collect();

    /*
    To help prioritize item rearrangement, every item type can be converted to a priority:

        Lowercase item types a through z have priorities 1 through 26.
        Uppercase item types A through Z have priorities 27 through 52.

    In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

    Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    */

    let mut priority_sum = 0;
    //for each line, split in half
    //make hashset of first half, iterate over 2nd half to find match
    //add int value of char to priority sum

    for rucksack in rucksacks{
        let mut matched = 0;
        //create the hash set of the first half
        let sack1:HashSet<char> = rucksack[0..rucksack.len()/2].chars().collect();
        //find which chars from second half are in first half
        for c in rucksack[rucksack.len()/2 .. rucksack.len()].chars(){
            if sack1.contains(&c){
                matched = c as usize;
                if matched > 97{
                    matched -= 96;
                }else{
                    matched = matched -65  + 27
                }
                //println!("match found! {} with {} {} {}", c, matched, 'a' as usize, 'A' as usize );
                break;
            }
        }
        priority_sum += matched;
    }

    /*
    // fill the set with the characters from the shorter string
        let set: HashSet<char> = shorter.chars().collect();

        longer.chars().any(|c| set.contains(&c))
    */

    println!("The sum of priorities for part 1 is {}", priority_sum);

    
}

fn part_two(rucksacks: &Vec<String>){
    //convert Vec<String> to Vec<&str>
    let rucksacks: Vec<&str> = rucksacks.iter().map(|s| &**s).collect();

    /*
    To help prioritize item rearrangement, every item type can be converted to a priority:

        Lowercase item types a through z have priorities 1 through 26.
        Uppercase item types A through Z have priorities 27 through 52.

    In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

    Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    */

    let mut priority_sum = 0;
    //for each line, split in half
    //make hashset of first half, iterate over 2nd half to find match
    //add int value of char to priority sum

    for i in (0..rucksacks.len()-2).step_by(3){
        let r1 = rucksacks[i];
        let r2 = rucksacks[i+1];
        let r3 = rucksacks[i+2];
        //create the hash set of the first bag
        let sack1:HashSet<char> = r1.chars().collect();
        
        let mut temp_matches:Vec<char> = Vec::new();
        //find which chars from second bag are in first bag
        for c in r2.chars(){
            if sack1.contains(&c){
                temp_matches.push(c);
            }
        }

        //find which char from the first two is in the third bag
        let sack3:HashSet<char> = r3.chars().collect();
        let mut m:char = ' ';
        
        for c in temp_matches{
            if sack3.contains(&c){
                m = c;
                //println!("match! {}",c);
                break;
            }
        }


        let mut matched = m as usize;
        if matched > 97{
            matched -= 96;
        }else{
            matched = matched -65  + 27
        }

        priority_sum += matched;
    }

    /*
    // fill the set with the characters from the shorter string
        let set: HashSet<char> = shorter.chars().collect();

        longer.chars().any(|c| set.contains(&c))
    */

    println!("The sum of priorities for part 2 is {}", priority_sum);

    
}



//return the lines from the input file as a vector
fn process_input(filename:&str) -> Vec<String>{
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut return_vals: Vec<String> = Vec::new();
    
    for l in lines{
        return_vals.push( String::from(l) );
    }

    let return_vals = return_vals.clone();
    return_vals
}