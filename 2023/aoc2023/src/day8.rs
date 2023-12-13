use std::fs;

pub fn main(){
    //let filename = "./inputs/day8_test.txt";
    let filename = "./inputs/day8.txt";
    let (instructions, nodes,start_idx, final_idx) = process_input(filename);
    let (instructions2, nodes2,start_idx_vec, final_idx_vec) = process_input_2(filename);    
    
    println!("{:?}", instructions);  
    println!("{:?}", nodes);  
    println!("{:?}", final_idx);  
    //part1(instructions, nodes, start_idx, final_idx);
    part2(instructions2, nodes2, start_idx_vec, final_idx_vec);
    //part2(races.clone());
    
}

fn part1(instructions:Vec<usize>, nodes:Vec<Node>, start_idx:usize, final_idx:usize){
    //follow the commands to get to the node ZZZ
    println!("Part 1");


    let mut current_idx = start_idx;
    let mut n_moves = 0;
    let mut move_idx = 0;
    while current_idx != final_idx{
        println!("Current node: {:?}", nodes[current_idx]);
        let lr = instructions[move_idx];
        current_idx = nodes[current_idx].connections[lr];
        n_moves += 1;
        if move_idx >= instructions.len()-1{
            move_idx = 0;
        }else{
            move_idx += 1;
        }
    }
    println!("It took {} moves to get to {:?}", n_moves, nodes[current_idx]);
    
    
}



fn part2(instructions:Vec<usize>,nodes:Vec<Node>, start_idx:Vec<usize>, final_idx:Vec<usize>){
    //
    println!("Part 2");
    //follow the commands to get to the nodes ending in Z from the nodes ending in A
    //moves happen the same for all 
    let mut n_moves = 0;
    let mut move_idx = 0;
    let mut remaining_nodes:Vec<&Node> = start_idx.iter().map(|idx| &nodes[*idx]).collect();
    
    while remaining_nodes.len() > 0{
        
        let lr = instructions[move_idx];
        let mut indices_to_del:Vec<usize> = Vec::new();
        for i in 0..remaining_nodes.len(){
            let current_idx = remaining_nodes[i].connections[lr];    
            if final_idx.contains(&current_idx){
                //this walker has reached an end node, mark it
                indices_to_del.push(i);
            }
            remaining_nodes[i] = &nodes[current_idx];
        }

        n_moves += 1;
        if move_idx >= instructions.len()-1{
            move_idx = 0;
        }else{
            move_idx += 1;
        }
        if indices_to_del.len() == start_idx.len(){
            break;
        }
    }
    println!("It took {} moves to get to the end", n_moves);
    
    
}


#[derive(Debug, Clone)]
struct Node {
    name:String,
    idx:usize,
    connections: [usize;2],
}


fn process_input(filename:&str)->(Vec<usize>,Vec<Node>, usize, usize){
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    
    let mut nodes:Vec<Node> = Vec::new();
    let mut node_names:Vec<&str> = Vec::new();
    
    //get the left/right instruction pattern as index 0,1 of children
    let instructions:Vec<usize> = lines[0].trim().chars().map(|x| {
        if x=='R'{
            1
        }else{
            0
        }} ).collect();
    
    let mut final_idx = lines.len()-2;
    let mut start_idx:usize = 0;
    //first make the nodes with no children
    for i in 2..lines.len(){
        let name = &lines[i][..3];
        nodes.push(Node{name:String::from(name),idx:i-2,connections:[0,0]});
        node_names.push(&name);
        if name == "ZZZ"{
            final_idx = i - 2;
        }else if name=="AAA"{
            start_idx = i-2;
        }
    }

    //then backfill the children indices
    for i in 2..lines.len(){
        let name = &lines[i][..3];
        let child_left = &lines[i][7..10];
        let child_right = &lines[i][12..15];
        
        let idx_name = node_names.iter().position(|&r| r == String::from(name)).unwrap();
        let idx_left = node_names.iter().position(|&r| r == String::from(child_left)).unwrap();
        let idx_right = node_names.iter().position(|&r| r == String::from(child_right)).unwrap();

        nodes[idx_name].connections = [idx_left, idx_right];
    }

    return (instructions, nodes, start_idx, final_idx)

}

fn process_input_2(filename:&str)->(Vec<usize>,Vec<Node>, Vec<usize>, Vec<usize>){
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    
    let mut nodes:Vec<Node> = Vec::new();
    let mut node_names:Vec<&str> = Vec::new();
    
    //get the left/right instruction pattern as index 0,1 of children
    let instructions:Vec<usize> = lines[0].trim().chars().map(|x| {
        if x=='R'{
            1
        }else{
            0
        }} ).collect();
    
    let mut final_idx:Vec<usize> = Vec::new();
    let mut start_idx:Vec<usize> = Vec::new();
    //first make the nodes with no children
    for i in 2..lines.len(){
        let name = &lines[i][..3];
        nodes.push(Node{name:String::from(name),idx:i-2,connections:[0,0]});
        node_names.push(&name);
        if name.chars().nth(2) == Some('Z'){
            final_idx.push(i - 2);
        }else if name.chars().nth(2)==Some('A'){
            start_idx.push(i-2);
        }
    }

    //then backfill the children indices
    for i in 2..lines.len(){
        let name = &lines[i][..3];
        let child_left = &lines[i][7..10];
        let child_right = &lines[i][12..15];
        
        let idx_name = node_names.iter().position(|&r| r == String::from(name)).unwrap();
        let idx_left = node_names.iter().position(|&r| r == String::from(child_left)).unwrap();
        let idx_right = node_names.iter().position(|&r| r == String::from(child_right)).unwrap();

        nodes[idx_name].connections = [idx_left, idx_right];
    }

    return (instructions, nodes, start_idx, final_idx)

}