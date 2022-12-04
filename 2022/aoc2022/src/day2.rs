use std::fs;

pub fn main(){
    //let filename = "./inputs/day2_test.txt";
    let filename = "./inputs/day2.txt";
    let rounds = process_input(filename);

    part_one(&rounds);
    part_two(&rounds);

}

fn part_one(rounds: &Vec<String>){
    //convert Vec<String> to Vec<&str>
    let rounds: Vec<&str> = rounds.iter().map(|s| &**s).collect();
    let mut sum = 0;
    
    //simply iterate over all rounds (lines) and add score to the sum
    for m in rounds{
        let move1 = &m[0..1];
        let move2 = &m[2..3];

        sum += calculate_score(move1, move2);
    }

    println!("The total score for part 1 is {}",sum);

}

fn part_two(rounds: &Vec<String>){
    /*
    The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

    The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

        In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
        In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
        In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

    Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

    Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
    */

    let rounds: Vec<&str> = rounds.iter().map(|s| &**s).collect();
    let mut sum = 0;

    //for each round (line) compute the move based on move1 and result, and then add score to total
    for m in rounds{
        let move1 = &m[0..1];
        let result = &m[2..3];
        //need to determine move 2 based on move1 and result
        let move2 = determine_move(move1, result);

        sum += calculate_score(move1, &move2);
    }

    println!("The total score for part 2 is {}",sum);
}

//given move1 and move2 compute the score 
//points are given for tie and victory
//and each move has a point value associated with it
fn calculate_score(move1: &str, move2: &str) -> isize{
    let mut score = 0;
    

    let won = check_win(&move1, &move2);
    if won == 1{
        score += 3;
    }else if won == 2{
        score += 6;
    }

    if move2 == "X"{
        score += 1;
    }else if move2 == "Y"{
        score += 2;
    }else if move2 == "Z"{
        score += 3;
    }


    score
}

//check if move2 beats move1
//messy match structures... could likely be cleaned up
fn check_win(move1: &str, move2: &str) -> usize{

    if move1 == move2{
        return 1;
    }

    let won = match move1 {
        "A" => {

            if move2 == "Y"{
                2
            }else if move2 == "X"{
                1
            }else{
                0
            }
        },
        "B" => {
            if move2 == "Z"{
                2
            }else if move2 == "Y"{
                1
            }else{
                0
            }
            },
        "C" => {
            if move2 == "X"{
                2
            }else if move2 == "Z"{
                1
            }else{
                0
            }
        },
        _ => 0
    };

    won
}

//use match structures to determine move2 for the required scenario
fn determine_move(move1:&str, result:&str) -> String{
    
    let move2 = match move1{
        "A" => {
            match result{
                "X" => "Z",
                "Y" => "X",
                "Z" => "Y",
                _ => "",
            }
        },
        "B" => {
            match result{
                "X" => "X",
                "Y" => "Y",
                "Z" => "Z",
                _ => "",
            }
        },
        "C" => {
            match result{
                "X" => "Y",
                "Y" => "Z",
                "Z" => "X",
                _ => "",
            }
        },
        _ => " "
    };



    String::from(move2)
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