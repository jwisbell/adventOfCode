use std::fs;

pub fn main(){
    //let filename = "./inputs/day12_test.txt";
    let filename = "./inputs/day12.txt";
    let inputs = process_input(filename);
    println!("{:?}", inputs);
    
    //part1(inputs.clone());
    part2(inputs.clone());

    //println!("{}",superposition(inputs[2].clone()) );
    
}

fn part1(data:Vec<Vents>){
    //find the number of possible ways to represent the given sequences
    println!("Part 1");
    let mut all_counts:Vec<usize> = Vec::new();
    
    for row in data{
        let x = superposition(row);
        all_counts.push(x);
    }
    //println!("counts {:?}", all_counts);
    println!("The sum is {:?}", all_counts.iter().sum::<usize>());
}

fn part2(data:Vec<Vents>){
    // i think i have to do wave function collapse.....
    println!("Part 2");

    let mut all_counts:Vec<usize> = Vec::new();
    
    for row in data{
        println!("Working on row {:?}", row);
        let new_row = unfold(row); //is it possible to solve the subproblems?
        let x = superposition(new_row);
        println!("Possibilities in row {:?}", x);
        all_counts.push(x);
    }
    println!("counts {:?}", all_counts);
    println!("The sum is {:?}", all_counts.iter().sum::<usize>());
}


fn superposition(row:Vents)->usize{
    let mut stack:Vec<Vec<usize>> = Vec::new();
    stack.push(row.springs);   
    let mut count = 0;
    

    while stack.len() > 0{
        let current = stack.pop().unwrap();
        let mut temp_seq:Vec<usize> = row.sequence.clone(); //Vec::new();
        let known = get_known_sequences(&current);
        for k in known{
            if !row.sequence.contains(&k){
                temp_seq.push(k);
            }
        }

        let maxlen = row.sequence.iter().max().unwrap_or(&0);

        let uk = get_first_unknown(&current);
        
        if uk == current.len()+1{
            //println!("{}, {:?}, {:?}, {:?}", uk, current, temp_seq, get_known_sequences(&current));
            if evaluate_seq(&current, &row.sequence){
                count+=1;
            }
            continue;
        }
        
        let mut left = 0;
        let mut right = 0;
        if uk > 0 {
            left = current[uk-1];
        }
        if uk < current.len()-1{
            right = current[uk+1];
        }
        let combo = left*right;
        let mut copy1 = current.clone();
        copy1[uk] = 1;
        let mut copy0 = current.clone();
        copy0[uk] = 0;
        let _ = match combo {
            1 => {
                //see how far left and right 1s extend
                //look left 
                let mut idx_left = uk-1;
                let mut idx_right = uk+1;
                for i in (0..uk-1).rev(){
                    if current[i]!=1{
                        idx_left = i+1;
                        break;
                    }
                }
                for i in uk+1..current.len(){
                    if current[i]!=1{
                        idx_right = i-1;
                        break;
                    }
                }
                let length = idx_right-idx_left;
                //println!("{} {} {} {}", length, maxlen,idx_left, idx_right);
                if length > *maxlen{
                    //it must be a zero
                    stack.push(copy0);
                    continue;
                }else{
                    //both are valid
                    stack.push(copy1);
                    stack.push(copy0);
                    continue;
                }
            }, //both are 1
            0 => {
                if left==right{
                    if temp_seq.contains(&1){
                        //both values are okay
                        stack.push(copy1);
                        stack.push(copy0);
                        continue;
                    }else{//only 0 works
                        stack.push(copy0);
                        continue;
                    }
                }else{
                    //both values are okay
                    stack.push(copy1);
                    stack.push(copy0);
                    continue;
                }
            } 
            _=>{
                //both values are okay
                stack.push(copy1);
                stack.push(copy0);
                continue;
            } 
        };
    }
    count
}
 

fn get_known_sequences(springs:&Vec<usize>)->Vec<usize>{
    let mut local_seq:Vec<usize> = Vec::new();
    let mut current_run = 0;
    let mut in_run:bool = false;
    
    for i in 0..springs.len(){
        if springs[i] == 1{
            if in_run{
                current_run += 1;
            }else{
                in_run = true;
                current_run =1;
            }
        }else if springs[i] == 0{
            if in_run{
                local_seq.push(current_run);
                in_run = false;
                current_run = 0;
            }
        }
    }

    if in_run{
        local_seq.push(current_run);
    }

    return local_seq
}

fn unfold(input:Vents)->Vents{
    //To unfold the records, on each row, replace the list of spring conditions with five copies of itself (separated by ?) 
    //and replace the list of contiguous groups of damaged springs with five copies of itself (separated by ,).
    let mut springs:Vec<usize> = Vec::new();
    let mut seq:Vec<usize> = Vec::new();
    for i in 0..5{
        springs.append(&mut input.springs.clone());
        springs.push(2);

        seq.append(&mut input.sequence.clone());
    }

    return Vents { sequence: seq, springs: springs }
}

#[derive(Debug, Clone)]
struct Vents{
    sequence: Vec<usize>,
    springs: Vec<usize>, //convert ., #, ? to 0, 1, 2
}

fn get_first_unknown(springs: &Vec<usize>)->usize{
    for i in 0..springs.len(){
        if springs[i] == 2{
            return i;
        }
    }
    springs.len()+1
}

fn get_all_unknown(springs: &Vec<usize>)->Vec<usize>{
    let mut unknowns:Vec<usize> = Vec::new();
    for i in 0..springs.len(){
        if springs[i] == 2{
            unknowns.push(i);
        }
    }
    unknowns
}

fn generate_possibilities(current_attempt: Vec<usize>)->Vec<Vec<usize>>{
    //recursively generate the possibilities... 
    //each 2 node must return a 0 and 1 option
    let mut new_attempts:Vec<Vec<usize>> = Vec::new();
    if !current_attempt.contains(&2){
        new_attempts.push(current_attempt);
        return new_attempts;
    }
    let idx = get_first_unknown(&current_attempt);
    
    let mut temp0 = current_attempt.clone();
    temp0[idx]=0;
    new_attempts.append( &mut generate_possibilities(temp0.clone()) );

    let mut temp1 = current_attempt.clone();
    temp1[idx]=1;
    new_attempts.append( &mut generate_possibilities(temp1.clone()) );

    new_attempts
}



fn evaluate_seq(springs: &Vec<usize>, sequence:&Vec<usize>)->bool{
    let local_seq = get_known_sequences(springs);

    return local_seq == *sequence
}


fn process_input(filename:&str) -> Vec<Vents>{
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    
    let mut maps:Vec<Vents> = Vec::new();
    for line in lines{
        let parts:Vec<&str> = line.trim().split_whitespace().collect();
        let springs:Vec<usize> = parts[0].chars().map(|x| {
            match x {
                '?'=>2,
                '#'=>1,
                '.'=>0,
                _=>0
            }
        }).collect();
        let counts:Vec<usize> = parts[1].split(',').collect::<Vec<&str>>().iter().map(|x| x.parse::<usize>().unwrap()).collect();
        let vent = Vents{
            sequence:counts,
            springs:springs,
        };
        maps.push(vent);
    }
    maps

}
