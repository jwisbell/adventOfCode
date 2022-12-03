
use std::fs;

pub fn main(){
    //let filename = "./inputs/day10_test.txt";
    let filename = "./inputs/day10.txt";
    let syntax = process_file(filename);
    println!("{:?}", syntax);
    part_onetwo(&syntax);
}


fn part_onetwo(v: &Vec<String>){
    let mut score_one = 0;
    let mut scores_two:Vec<usize> = Vec::new();
    for line in v{
        let mut char_counter: [isize; 4] = [0;4]; //corresponding to (, [, {, < -- unused in the end
        let chars: Vec<char> = line.chars().collect();
        let mut wrong_char = ' '; 
        let mut char_stack: Vec<char> = Vec::new(); // stack to keep track of order of pairs
        //add each opening bracket to stack, when meeting closing bracket, see if it matches the most recent opening
        for c in chars{
            if char_stack.len() > 0{
                match c{
                    '(' => {char_counter[0] += 1; char_stack.push(c); },
                    ')' =>  {
                                char_counter[0] -= 1; 
                                if char_stack[char_stack.len()-1] == '('{
                                    char_stack.pop();
                                }else{
                                    wrong_char = c;
                                    break;
                                }
                            },
                    '[' => {char_counter[1] += 1; char_stack.push(c);},
                    ']' => {
                                char_counter[1] -= 1; 
                                if char_stack[char_stack.len()-1] == '['{
                                    char_stack.pop();
                                }else{
                                    wrong_char = c;
                                    break;
                                }
                            },
                    '{' => {char_counter[2] += 1; char_stack.push(c);},
                    '}' => {
                                char_counter[2] -= 1; 
                                if char_stack[char_stack.len()-1] == '{'{
                                    char_stack.pop();
                                }else{
                                    wrong_char = c;
                                    break;
                                }
                            },
                    '<' => {char_counter[3] += 1; char_stack.push(c);},
                    '>' => {
                                char_counter[3] -= 1; 
                                if char_stack[char_stack.len()-1] == '<'{
                                    char_stack.pop();
                                }else{
                                    wrong_char = c;
                                    break;
                                }
                            },
                    _ => {},
                }
            }else{ char_stack.push(c);}
            
        }

        if wrong_char != ' '{ //for part one
            score_one += match wrong_char{
                '('|')'=>3,
                '['|']'=>57,
                '{'|'}'=>1197,
                '<'|'>'=>25137,
                _ => 0,
            };
        }else { // just incomplete, for part 2
            //map the openings (in order) into closings
            let t: Vec<char> = char_stack.into_iter().rev().map(|x| match x {
                '('=>')',
                '['=>']',
                '{'=>'}',
                '<'=>'>',
                _ => ' ',
            }   ).collect();
            let mut temp_score = 0;
            for c in t{
                temp_score = temp_score * 5;
                temp_score += match c{
                    ')'=>1,
                    ']'=>2,
                    '}'=>3,
                    '>'=>4,
                    _ => 0,
                };
            }
            sorted_insert(&mut scores_two, temp_score);
        }
    }

    println!("Part one: {}", score_one);
    println!("Part two: {}", scores_two[scores_two.len()/2  ]  );
}

//create a sorted list while reading in the data
fn sorted_insert(v: &mut Vec<usize>, x:usize){
    //find the first element that the number is greater than, insert there
    if v.len() == 0{
        v.push(x);
        return ();
    }
    if x < v[0]{ // number less than first element, insert there
        v.insert(0,x);
        return ();
    }else{
        for i in 1..v.len(){
            if x < v[i]{ //number less than a given element, insert there
                v.insert(i,x   );
                return ();
            }
        }
    }
    // number is greater than other existing ones, add to end
    v.push(x);
    
}

//read the input data, splitting into Display objects
fn process_file(filename: &str) -> Vec<String>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut v : Vec<String> = Vec::new();
    //should have one line per display
    for l in lines{        
        v.push( String::from( l.trim() )  );
    }
    v
}