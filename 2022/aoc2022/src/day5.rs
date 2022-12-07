use std::fs;

pub fn main(){
    //let filename = "./inputs/day5_test.txt";
    let filename = "./inputs/day5.txt";
    let (instructions, stacks) = process_input(filename);

    //println!("stacks:{:?}, instructions:{:?}", stacks, instructions);

    part_one(&stacks, &instructions);
    part_two(&stacks, &instructions);
    //part_two(&assignments);

}

fn part_one(stacks: &[Vec<char>; 10], instructions: &Vec<Instruction> ){
    //i think we just execute the instructions and read the top crate in each stack
    let mut local_stacks = stacks.clone();
    for i in instructions{
        local_stacks = execute(local_stacks, i);
    }

    //println!("stacks:{:?}, instructions:{:?}", local_stacks, instructions);
    let mut answer:Vec<char> = Vec::new();
    
    for s in local_stacks{
        let mut local = s.clone();
        if local.len() > 0{
            answer.push(local.pop().unwrap());
        }
    }

    let final_answer: String = answer.iter().collect();
    println!("{:?}", final_answer )
}

fn part_two(stacks: &[Vec<char>; 10], instructions: &Vec<Instruction> ){
    //i think we just execute the instructions and read the top crate in each stack
    let mut local_stacks = stacks.clone();
    for i in instructions{
        local_stacks = execute_two(local_stacks, i);
    }

    //println!("stacks:{:?}, instructions:{:?}", local_stacks, instructions);
    let mut answer:Vec<char> = Vec::new();
    
    for s in local_stacks{
        let mut local = s.clone();
        if local.len() > 0{
            answer.push(local.pop().unwrap());
        }
    }

    let final_answer: String = answer.iter().collect();
    println!("{:?}", final_answer )
}


#[derive(Debug)]
struct Instruction{
    amount: usize,
    origin: usize,
    destination: usize
}

impl Instruction{
    //do stuff here?
}

fn execute(mut stacks: [Vec<char>; 10], instruction: &Instruction) -> [Vec<char>; 10]{

    let origin = instruction.origin;
    let destination = instruction.destination;
    let amount = instruction.amount;
    let mut count = 0;
    while count < amount{

        let x = stacks[origin-1].pop().unwrap();
        stacks[destination-1].push(x);
        count  += 1;
    }


    return stacks;
}

fn execute_two(mut stacks: [Vec<char>; 10], instruction: &Instruction) -> [Vec<char>; 10]{

    let origin = instruction.origin;
    let destination = instruction.destination;
    let amount = instruction.amount;
    let mut count = 0;
    //now need to take "amount" from origin to place at dest
    let mut queue: Vec<char> = Vec::new();
    while count < amount{

        let x = stacks[origin-1].pop().unwrap();
        queue.push(x);
        count  += 1;
    }

    while queue.len() >0{
        let x = queue.pop().unwrap();
        stacks[destination-1].push(x);
    }

    return stacks;
}


//return the lines from the input file as a vector
fn process_input(filename:&str) -> (Vec<Instruction>,[Vec<char>; 10]){
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut crate_lines: Vec<String> = Vec::new();
    let mut instruction_lines: Vec<String> = Vec::new();
    
    let mut reading_crates = true;
    for l in lines{
        if l == ""{
            reading_crates = false;
            continue;
        }
        
        if reading_crates{
            crate_lines.push(String::from(l));
        }else{
            instruction_lines.push(String::from(l));
        }
    }

    let row_names_raw:String = String::from(crate_lines.pop().unwrap());
    let mut column_pos:Vec<usize> = Vec::new();
    for i in 0..row_names_raw.len(){
        if row_names_raw.chars().nth(i).unwrap() != ' '{
            column_pos.push(i);
        }
    }

    let mut all_stacks: [Vec<char>; 10] = Default::default();

    println!("The columns are located at {:?}", column_pos);

    //now i want to parse the crate lines into vectors 
    while crate_lines.len() > 0{
        let x = crate_lines.pop().unwrap();
        
        for i in 0..column_pos.len(){
            if x.chars().nth(column_pos[i]).unwrap() != ' '{
                all_stacks[i].push( x.chars().nth(column_pos[i]).unwrap() );
            }
        }
    }

    let mut inst: Vec<Instruction> = Vec::new();
    //and then parse the instructions into Instruction objects
    for il in instruction_lines{
        let parts: Vec<&str> = il.split(' ').collect();
        let amount:usize = parts[1].parse::<usize>().unwrap();
        let origin:usize = parts[3].parse::<usize>().unwrap();
        let destination:usize = parts[5].parse::<usize>().unwrap();
        inst.push( Instruction{amount:amount, origin:origin, destination:destination} );
    }

    return (inst, all_stacks)
}