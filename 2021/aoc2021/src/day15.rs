use std::fs;
use std::collections::HashMap;

pub fn main(){
    //let filename = "./inputs/day15_test.txt";
    let filename = "./inputs/day15.txt";
    let risks = process_file(filename);
    //println!("{:?}", risks[9][9]);
    //let coords = part_one(&depths);
    //part_two(&depths, &coords);
    let start: Vec<usize> = vec!(0,0);
    let goal: Vec<usize> = vec!(9,9);
    //part_one(&risks, &start,&goal);
    part_two(&risks, &start,&goal);
}

fn part_one(map:&Vec<Vec<usize>>, start: &Vec<usize>, goal: &Vec<usize> ){
    a_star(&map, &start, &goal);
}

fn part_two(map:&Vec<Vec<usize>>, start: &Vec<usize>, goal: &Vec<usize> ){

    //need to expand the map
    let mut new_map: Vec<Vec<usize>> = Vec::new();
    for row in map{
        let mut my_row = row.clone();
        let _ = &my_row.append(&mut vector_add(&row, 1));
        let _ = &my_row.append(&mut vector_add(&row, 2));
        let _ = &my_row.append(&mut vector_add(&row, 3));
        let _ = &my_row.append(&mut vector_add(&row, 4));
        //println!("{:?}",  my_row);
        new_map.push( my_row   );// .concat(vector_add(row, 2)).concat(vector_add(row, 3)).concat(vector_add(row, 4)) );
        
    }
    for j in 1..5{
        
        for i in 0..map.len(){
            let mut my_row = new_map[i].clone();        
            new_map.push( vector_add(&my_row, j)   );// .concat(vector_add(row, 2)).concat(vector_add(row, 3)).concat(vector_add(row, 4)) );
            //println!("{:?}",  vector_add(&my_row, j));
        } 
    }
      
    //println!("{:?}", new_map );
    let goal: Vec<usize> = vec!(new_map.len()-1 ,new_map.len()-1);
    a_star(&new_map, &start, &goal);
}

fn vector_add(arr: &Vec<usize>, y:usize) -> Vec<usize>{
    let r: Vec<usize> = arr.into_iter().map(
        |x| match x+y > 9{
                true => (x+y)%9,
                _ => x+y,
                }   
        ).collect();
    r
}

// A* finds a path from start to goal.
// h is the heuristic function. h(n) estimates the cost to reach goal from node n.
fn a_star( map: &Vec<Vec<usize>>, start: &Vec<usize>, goal: &Vec<usize>){
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set:Vec<Vec<usize>> = Vec::new();
    
    open_set.push(start.to_vec());

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from: HashMap< Vec<usize>, Vec<usize>  > = HashMap::new();
    came_from.insert(start.to_vec(),start.to_vec());

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score: HashMap< Vec<usize>, usize  > = HashMap::new();
    g_score.insert(start.to_vec(), 0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how short a path from start to finish can be if it goes through n.
    let mut f_score: HashMap< Vec<usize>, usize  > = HashMap::new();
    f_score.insert(start.to_vec(), h(&start.to_vec(), &goal.to_vec())  );

    while open_set.len() > 0{
        let index = argmin(&open_set, &f_score);
        let temp_set = open_set.clone();
        let current = temp_set.get( index ).unwrap() ;
        //println!("{:?}",current);
        if current == goal{
            //let temp = reconstruct_path(&came_from, &current,&start);
            println!("Part one answer: {:?}",g_score.get(&current.to_vec()).unwrap() );
            //println!("Part one path: {:?}", temp);
            break;
        }

        
        //for each neighbor of current
        let mut neighbors: Vec< Node > = Vec::new();
        let mut combos: Vec<Vec<usize>> = Vec::new();
        let x = *&current[1];
        let y = *&current[0];
        //get all the adjacent indices
        if x > 0 && y > 0 && x < map[0].len()-1 && y < map[0].len()-1{
            combos.push(  vec!(y-1,x)  );
            combos.push(  vec!(y,x+1)  );
            combos.push(  vec!(y+1,x)  );
            combos.push(  vec!(y,x-1)  );
        }else if y==0{ //top row
            if x==0{
                combos.push(  vec!(y,x+1)  );
                combos.push(  vec!(y+1,x)  );
            }else if x == map[0].len()-1{
                combos.push(  vec!(y+1,x)  );
                combos.push(  vec!(y,x-1)  );

            }else{
                combos.push(  vec!(y,x+1)  );
                combos.push(  vec!(y+1,x)  );
                combos.push(  vec!(y,x-1)  );
            }
        }else if y==map.len()-1{ //bottom row
            if x==0{
                combos.push(  vec!(y-1,x)  );
                combos.push(  vec!(y,x+1)  );
            }else if x == map[0].len()-1{
                combos.push(  vec!(y-1,x)  );
                combos.push(  vec!(y,x-1)  );
            }else{
                combos.push(  vec!(y-1,x)  );
                combos.push(  vec!(y,x+1)  );
                combos.push(  vec!(y,x-1)  );
            }
        }else if x==map[0].len()-1{ //right column
            if y==0{
                combos.push(  vec!(y+1,x)  );
                combos.push(  vec!(y,x-1)  );
            }else if y == map.len()-1{
                combos.push(  vec!(y-1,x)  );
                combos.push(  vec!(y,x-1)  );
            }else{
                combos.push(  vec!(y-1,x)  );
                combos.push(  vec!(y+1,x)  );
                combos.push(  vec!(y,x-1)  );
            }
        }else if x==0{ //left column
            if y==0{
                combos.push(  vec!(y,x+1)  );
                combos.push(  vec!(y+1,x)  );
            }else if y == map.len()-1{
                combos.push(  vec!(y-1,x)  );
                combos.push(  vec!(y,x+1)  );
            }else{
                combos.push(  vec!(y-1,x)  );
                combos.push(  vec!(y,x+1)  );
                combos.push(  vec!(y+1,x)  );
            }
        }

        
        for c in combos{
            let mut temp_node = Node{
                loc: c,
                prev: current.to_vec(),
                cost: 0,
            };
            //temp_node.compute_cost(&map);
            neighbors.push(temp_node);
        }

        //println!("{:?}",neighbors);
        for n in neighbors{
            let d = map[n.loc[0]][n.loc[1]];
            let tentative_g_score = g_score.get(&current.to_vec()).unwrap() + d;
            if g_score.contains_key(&n.loc ){
                if *g_score.get(&n.loc).unwrap() > tentative_g_score{ //this is the best path to this node
                    let mut x = g_score.get_mut(&n.loc).unwrap();
                    *x = tentative_g_score;
                    
                    if came_from.contains_key(&n.loc ){
                        let mut x = came_from.get_mut(&n.loc).unwrap();
                        *x = current.to_vec();
                    }else{
                        came_from.insert(n.loc.to_vec(), current.to_vec());
                    }

                    if f_score.contains_key(&n.loc ){
                        let mut x = f_score.get_mut(&n.loc).unwrap();
                        *x = tentative_g_score + h(&n.loc, &goal.to_vec());
                    }else{
                        f_score.insert(n.loc.to_vec(), tentative_g_score + h(&n.loc, &goal.to_vec()));
                    }

                    if !open_set.contains( &n.loc ){  open_set.push(n.loc.to_vec() ); }

                }
            }else{
                g_score.insert(n.loc.to_vec(), tentative_g_score);
                f_score.insert(n.loc.to_vec(), tentative_g_score + h(&n.loc, &goal.to_vec()));
                came_from.insert(n.loc.to_vec(), current.to_vec());
                if !open_set.contains( &n.loc ){  open_set.push(n.loc.to_vec() ); }
            }

        }
        //println!("{:?}",open_set);
        open_set.remove(index);

    }


}


fn reconstruct_path(came_from: &HashMap<Vec<usize>, Vec<usize>>, current: &Vec<usize>, start:&Vec<usize>  ) -> Vec<Vec<usize>>{
    let mut path: Vec<Vec<usize>> = Vec::new();
    path.push(current.to_vec());
    let mut my_current = current;
    while came_from.contains_key(my_current){
        my_current = came_from.get(my_current).unwrap();
        
        path.insert(0,my_current.to_vec());
        if my_current == start{
            break;
        }

    }
    path
}

fn argmin( arr: &Vec<Vec<usize>>, costs:&HashMap<Vec<usize>,usize > ) -> usize{
    let mut min_val = 1000000;
    let mut min_index =  0;
    for i in 0..arr.len(){
        let node = &arr[i];
        if *costs.get(&node.to_vec()).unwrap() < min_val{
            min_val =  *costs.get(&node.to_vec()).unwrap();
            min_index = i;
        }
    }
    min_index
}

fn h(loc: &Vec<usize>, end:&Vec<usize>) -> usize{
    end[0] - loc[0] + end[1] - loc[1]
}

#[derive(Debug)]
struct Node{
    loc: Vec<usize>,
    prev: Vec<usize>,
    cost: usize,
}

impl Node{
    
}


//read the input data, splitting into 2d map
fn process_file(filename: &str) -> Vec<Vec<usize>>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut v : Vec<Vec<usize>> = Vec::new();
    //should have one line per display
    for l in lines{
        let temp:Vec<&str> = l.trim().split("").collect();
        println!("{:?}", temp);
        let x:Vec<usize> = temp[1..temp.len()-1].into_iter().map( |x| x.trim().parse::<usize>().unwrap()  ).collect();
        v.push(x);
    }
    v
}