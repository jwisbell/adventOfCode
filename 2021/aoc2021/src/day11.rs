use std::fs;
use std::collections::HashMap;

pub fn main(){
    //let filename = "./inputs/day11_test.txt";
    let filename = "./inputs/day11.txt";
    let mut squid = process_file(filename);
    println!("{:?}",squid);
    
    part_onetwo(&mut squid);
}


fn part_onetwo(map: &mut Vec<Vec<usize>>){
    let mut total_flashes = 0;
    let n_steps = 500;
    for i in 0..n_steps{
        let mut lit: HashMap<String,bool > = HashMap::new();//coords formatted as "x,y"
        for y in 0..map.len(){
            for x in 0..map[y].len(){
                if map[y][x] < 9{
                    map[y][x] += 1;
                }else{
                    //if unlit, add to hashmap and change adj values recursively
                    if !lit.contains_key(&String::from(format!("{},{}",x,y ))){
                        lit.insert( String::from(format!("{},{}",x,y )),true);
                        change_adj(x,y, map, &mut lit);
                    }
                }
            }
        }

        // set illum to 0
        for (key,_value) in lit.iter(){
            let coords: Vec<usize> = key.split(",").collect::<Vec<&str>>().into_iter().map(|s| s.parse::<usize>().unwrap() ).collect();
            let y = coords[1];
            let x = coords[0];
            map[y][x] = 0;

        }

        // for part 2 calculate if all illum
        if(lit.len() == map[0].len() * map.len()){
            println!("All illum @step {}", i+1);
            break;
        }
        
        total_flashes += lit.len();
    }

    println!("Total N illum: {}", total_flashes);
}

//recursive func to change values of squid adj to flashes
fn change_adj(x:usize,y:usize,map: &mut Vec<Vec<usize>>,tracking_arr:&mut HashMap<String,bool >){
    
    let mut combos:Vec<Vec<usize>> = Vec::new();
    //get all the adjacent indices
    if x > 0 && y > 0 && x < map[0].len()-1 && y < map[0].len()-1{
        combos.push(  vec!(y-1,x-1)  );
        combos.push(  vec!(y-1,x)  );
        combos.push(  vec!(y-1,x+1)  );
        combos.push(  vec!(y,x+1)  );
        combos.push(  vec!(y+1,x+1)  );
        combos.push(  vec!(y+1,x)  );
        combos.push(  vec!(y+1,x-1)  );
        combos.push(  vec!(y,x-1)  );
    }else if y==0{ //top row
        if x==0{
            combos.push(  vec!(y,x+1)  );
            combos.push(  vec!(y+1,x+1)  );
            combos.push(  vec!(y+1,x)  );
        }else if x == map[0].len()-1{
            combos.push(  vec!(y+1,x)  );
            combos.push(  vec!(y+1,x-1)  );
            combos.push(  vec!(y,x-1)  );

        }else{
            combos.push(  vec!(y,x+1)  );
            combos.push(  vec!(y+1,x+1)  );
            combos.push(  vec!(y+1,x)  );
            combos.push(  vec!(y+1,x-1)  );
            combos.push(  vec!(y,x-1)  );
        }
    }else if y==map.len()-1{ //bottom row
        if x==0{
            combos.push(  vec!(y-1,x)  );
            combos.push(  vec!(y-1,x+1)  );
            combos.push(  vec!(y,x+1)  );
        }else if x == map[0].len()-1{
            combos.push(  vec!(y-1,x-1)  );
            combos.push(  vec!(y-1,x)  );
            combos.push(  vec!(y,x-1)  );
        }else{
            combos.push(  vec!(y-1,x-1)  );
            combos.push(  vec!(y-1,x)  );
            combos.push(  vec!(y-1,x+1)  );
            combos.push(  vec!(y,x+1)  );
            combos.push(  vec!(y,x-1)  );
        }
    }else if x==map[0].len()-1{ //right column
        if y==0{
            combos.push(  vec!(y+1,x)  );
            combos.push(  vec!(y+1,x-1)  );
            combos.push(  vec!(y,x-1)  );
        }else if y == map.len()-1{
            combos.push(  vec!(y-1,x-1)  );
            combos.push(  vec!(y-1,x)  );
            combos.push(  vec!(y,x-1)  );
        }else{
            combos.push(  vec!(y-1,x-1)  );
            combos.push(  vec!(y-1,x)  );
            combos.push(  vec!(y+1,x)  );
            combos.push(  vec!(y+1,x-1)  );
            combos.push(  vec!(y,x-1)  );
        }
    }else if x==0{ //left column
        if y==0{
            combos.push(  vec!(y,x+1)  );
            combos.push(  vec!(y+1,x+1)  );
            combos.push(  vec!(y+1,x)  );
        }else if y == map.len()-1{
            combos.push(  vec!(y-1,x)  );
            combos.push(  vec!(y-1,x+1)  );
            combos.push(  vec!(y,x+1)  );
        }else{
            combos.push(  vec!(y-1,x)  );
            combos.push(  vec!(y-1,x+1)  );
            combos.push(  vec!(y,x+1)  );
            combos.push(  vec!(y+1,x+1)  );
            combos.push(  vec!(y+1,x)  );
        }
    }

    //for each adj loc, check value and flash if necessary
    for c in combos{
        if map[c[0]][c[1]] < 9 { 
            map[c[0]][c[1]] += 1;   
        }
        else{
            //recurse here
            if !tracking_arr.contains_key(&String::from(format!("{},{}",c[1],c[0] ))){
                tracking_arr.insert( String::from(format!("{},{}",c[1],c[0] )),true);
                change_adj(c[1],c[0], map, tracking_arr);
            }
        }
    }
    //base case do nothing
}

//read the input data, splitting into map of squid
fn process_file(filename: &str) -> Vec<Vec<usize>>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut v : Vec<Vec<usize>> = Vec::new();
    for l in lines{
        let temp:Vec<&str> = l.trim().split("").collect();
        let x:Vec<usize> = temp[1..temp.len()-1].into_iter().map( |x| x.trim().parse::<usize>().unwrap()  ).collect();
        v.push(x);
    }
    v
}