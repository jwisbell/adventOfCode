use std::fs;

pub fn main(){
    //let filename = "./inputs/day2_test.txt";
    let filename = "./inputs/day2.txt";
    let turns = process_input(filename);

    let limit = Draw{red:12,green:13,blue:14};
    part1(turns.clone(), limit);
    
    part2(turns);
}

fn part1(turns: Vec<Vec<Draw>>, limit:Draw){
    let mut good_turns:Vec<usize> = Vec::new();
    for i in 0..turns.len(){
        let turn = &turns[i];
        
        let valid:Vec<usize> = turn.iter().map(|x| {
            if x.red <= limit.red && x.blue <= limit.blue && x.green <= limit.green{
                1
            }else{
                0
            }
        }).collect();
        
        let sum:usize = valid.iter().sum();
        if sum  == valid.len(){
            good_turns.push(i+1);
        }
    }

    println!("Part 1");
    let sum:usize = good_turns.iter().sum();
    println!("The sum of the good turns is {}", sum);
}



fn part2(turns: Vec<Vec<Draw>>){
    let mut game_power:Vec<usize> = Vec::new();
    
    for i in 0..turns.len(){
        let turn = &turns[i];
        //for each turn, find the max for each of the three values
        let red = max(turn.iter().map(|x| x.red).collect());
        let green = max(turn.iter().map(|x| x.green).collect());
        let blue = max(turn.iter().map(|x| x.blue).collect());
        game_power.push(red*green*blue);
    }

    println!("Part 2");
    let sum:usize = game_power.iter().sum();
    println!("The sum of the powers is {}", sum);
}

#[derive(Debug, Clone)]
struct Draw{
    red:usize,
    green:usize,
    blue:usize
}

fn max(v: Vec<usize>)->usize{
    let mut idx = 0;
    let mut value = v[idx];
    for i in 1..v.len(){
        if v[i] > value{
            value = v[i];
            idx = i;
        }
    }
    value
}

fn process_input(filename:&str)->Vec<Vec<Draw>>{
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    //save each Game as a vector of N draws
    let mut draws: Vec<Vec<Draw>> = Vec::new();
    for l in lines{
        let string: Vec<&str> = l.split(':').collect();
        let parts: Vec<&str> = string[1].trim().split(';').collect();
        let results:Vec<Draw> = parts.iter().map(|x| {
            let values: Vec<&str> = x.split(" ").collect();
            //find the number preceding each red, green, blue
            Draw{
                red:{
                        let p = values.iter().position(|&r| r.contains("red")).unwrap_or(values.len()); //return out of bounds if value not present
                        if p<values.len(){ //if the value exists
                            values[p-1].parse::<usize>().unwrap_or(0)
                        }else{
                            0
                        }
                    },
                green:{
                        let p = values.iter().position(|&r| r.contains("green")).unwrap_or(values.len());
                        if p<values.len(){ //if the value exists
                            values[p-1].parse::<usize>().unwrap_or(0)
                        }else{
                            0
                        }
                    },
                blue: {
                        let p = values.iter().position(|&r| r.contains("blue")).unwrap_or(values.len());
                        if p<values.len(){ //if the value exists
                            values[p-1].parse::<usize>().unwrap_or(0)
                        }else{
                            0
                        }
                    }, 
            }
        }).collect();

        draws.push(results);
    }
    draws
}