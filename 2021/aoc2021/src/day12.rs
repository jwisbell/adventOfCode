use std::fs;
use std::collections::HashMap;

pub fn main(){
    //let filename = "./inputs/day12_test.txt";
    let filename = "./inputs/day12.txt";
    let mut nodes = process_file(filename);
    println!("{:?}",nodes);
    
    part_onetwo(&mut nodes);
}


fn part_onetwo(map: &HashMap<String, Vec<String>>){
    let mut nodes: Vec<String> = Vec::new();
    for n in map.get(&String::from("start")).unwrap(){
        nodes.push(n.to_string());
    }

    //part 1
    let mut all_paths: Vec<Vec<String>> =  Vec::new();
    for node in &nodes{
        let mut visited: Vec<String> =  Vec::new();
        let mut counter: HashMap<String, usize> = HashMap::new();
        dfs(&node, &map, &mut visited, &mut counter, &mut all_paths, true);
    }
    
    println!("Part one: {}", all_paths.len());

    //part 2
    let mut all_paths: Vec<Vec<String>> =  Vec::new();
    for node in &nodes{
        let mut visited: Vec<String> =  Vec::new();
        let mut counter: HashMap<String, usize> = HashMap::new();
        dfs(&node, &map, &mut visited, &mut counter, &mut all_paths, false);
    }
    
    println!("Part two: {}", all_paths.len());
}

//depth-first search with some added fluff
fn dfs(start_node:&str, map: &HashMap<String, Vec<String>>, visited: &mut Vec<String>, counter:&mut HashMap<String, usize>, all_paths: &mut Vec<Vec<String>>, p1:bool) {
    //base case
    if start_node.to_string() == String::from("end") {
        return;
    }

    //otherwise add node to path
    visited.push(start_node.to_string());
    //add value to counter (for part 2)
    if !counter.contains_key( &start_node.to_string()){
        counter.insert(start_node.to_string(),  1  );
    }else{
        let x = counter.get_mut( &start_node.to_string()  ).unwrap();
        *x += 1;
    }

    //compute the possible nodes
    let mut possible: Vec<String> = Vec::new();
    for n in map.get(&start_node.to_string()).unwrap(){
        if n.to_string() != String::from("start"){
            if n.to_string() == n.to_lowercase(){
                //have to check if a small cave has only been visited once
                if p1{
                    if !visited.contains(n) || n.to_string() == String::from("end"){
                        possible.push(n.to_string());
                    }
                }else{//in part 2 need to modify so that up to one small cave can be visited twice
                    let mut only_one = true;
                    for (key, value) in counter.iter(){
                        if key.to_string() == key.to_lowercase(){
                            if *value > 1 {only_one = false;}
                        }
                    }
                    if !visited.contains(n) || only_one  || n.to_string() == String::from("end"){
                        possible.push(n.to_string());
                    }
                }
            }else{ // always add big caves to path
                possible.push(n.to_string());
            }
        }
        
    }
    //dead end, return
    if possible.len() == 0{
        return;
    }
    
    //launch recursive searches for each path
    let mut paths: Vec< Vec<String>> = Vec::new();
    for p in possible{
        if p == String::from("end"){
            //end of the line, add this to our array
            all_paths.push(visited.clone().to_vec()); 
        }else{
            //keep going! gotta go deeper
            dfs(&p,map, &mut visited.clone(), &mut counter.clone(), all_paths, p1  );  
        }   
    }
    return;
}



//read the input data, splitting into hashmap of Node, (connections)
fn process_file(filename: &str) -> HashMap<String, Vec<String>>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut v : HashMap<String, Vec<String>> = HashMap::new();
    for l in lines{
        let temp:Vec<&str> = l.trim().split("-").collect();

        //add both x -> y and y -> x
        if !v.contains_key( &String::from(temp[0])){
            v.insert(String::from(temp[0]),  vec!(String::from(temp[1]) )  );
        }else{
            let x = v.get_mut( &String::from(temp[0])  ).unwrap();
            x.push(String::from(temp[1]));
        }
        if !v.contains_key( &String::from(temp[1])){
            v.insert(String::from(temp[1]),  vec!(String::from(temp[0]) )  );
        }else{
            let x = v.get_mut( &String::from(temp[1])  ).unwrap();
            x.push(String::from(temp[0]));
        }
    }
    v
}