use std::{fs, usize};

pub fn main(){
    //let filename = "./inputs/day4_test.txt";
    let filename = "./inputs/day4.txt";
    let cards = process_input(filename);    
        
    part1(cards.clone());
    part2(cards);
}

fn part1(arr:Vec<(Vec<usize>, Vec<usize>)>){
    //iterate over each card and count up the number of matches
    //the score for that card is 2^(n-1) where n is number of matches
    //sum up all scores
    let mut scores:Vec<u32> = Vec::new();

    for card in arr{
        let count = count_matches(&card.0, &card.1);
        let base:u32 = 2;
        if count > 0{
            println!("Card {:?} | {:?} has {} matches with score {}", card.0, card.1, count, base.pow(count-1));
            scores.push(base.pow(count-1) ); 
        }
    }

    println!("Part 1");
    println!("The card scores are {:?}", scores);
    println!("The sum is {}", scores.iter().sum::<u32>() );
}

fn count_matches(winning:&Vec<usize>, mine:&Vec<usize>)->u32{
    let mut count:u32 = 0;
    //in the most naive way, look for matching numbers
    for n1 in winning{
        for n2 in mine{
            if n1==n2{
                count = count + 1;
            }
        }
    }
    return count
}

fn part2(arr:Vec<(Vec<usize>, Vec<usize>)>){
    //iterate over each card and count up the number of matches
    //the number of matches tells us how many new cards we get -- the next n cards are copied and evaluated
    //this occurs recursively until a card has no matches
    //count up the total number of scratch cards evaluated
    let mut total_cards = 0;
    for i in 0..arr.len(){
        let sum = recursive_check(i, &arr);
        total_cards += sum;
    }
    println!("Part 2");
    println!("There are {} cards", total_cards)
}


fn recursive_check(idx:usize, all_cards:&Vec<(Vec<usize>, Vec<usize>)>)->usize{
    //recursively look at the cards that are won by an individual card due to the copying mechanic
    let mut num_cards:usize = 1;
    let row = &all_cards[idx];
    let count = count_matches(&row.0, &row.1) as usize;
    //println!("Looking at card {} with {} matches", idx+1, count);
    if count > 0{
        for i in idx+1..idx+count+1{
            num_cards += recursive_check(i, all_cards);
        }
    }
    num_cards
}


fn process_input(filename:&str)->Vec<(Vec<usize>, Vec<usize>)>{
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    //save each line as a tuple of winning numbers and my numbers
    let mut rows: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
    for l in lines{
        let halves:Vec<&str> = l.trim().split(":").collect::<Vec<&str>>()[1].split("|").collect();

        let winning_numbers:Vec<usize> = halves[0].trim().split_whitespace().collect::<Vec<&str>>()
            .iter().map(|x| x.trim().parse::<usize>().unwrap_or(0)).collect();

        let my_numbers:Vec<usize> = halves[1].trim().split_whitespace().collect::<Vec<&str>>()
            .iter().map(|x| x.trim().parse::<usize>().unwrap_or(0)).collect();

        rows.push((winning_numbers, my_numbers));

    }
    rows
}