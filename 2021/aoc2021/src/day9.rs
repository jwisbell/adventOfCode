use std::fs;
use std::collections::HashMap;

pub fn main(){
    //let filename = "./inputs/day9_test.txt";
    let filename = "./inputs/day9.txt";
    let depths = process_file(filename);

    let coords = part_one(&depths);
    part_two(&depths, &coords);
}

fn part_one(v: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let mut sum = 0;

    let mut coords = Vec::new();
    
    for i in 0..v.len(){
        for j in 0..v[i].len(){
            let val = v[i][j];
            let is_min = {
                let top = match i{
                    0 => true,
                    _=> val < v[i-1][j]
                };
                let left = match j{
                    0 => true,
                    _=> val < v[i][j-1]
                };
                let right = {
                    if j == v[0].len()-1{
                        true
                    }
                    else{
                        val < v[i][j+1]
                    }
                }; 
                let bottom = {
                    if i == v.len()-1{
                        true
                    }
                    else{
                        val < v[i+1][j]
                    }
                }; 
                top && left && right && bottom
            };
            if is_min{
                sum += val + 1;
                coords.push( vec!(i,j)   );
            }
        }
    }

    println!("Part one answer: {}", sum);
    coords

}

fn part_two(v: &Vec<Vec<usize>>, coords: &Vec<Vec<usize>>){
    let mut basin_sizes: Vec<usize> = Vec::new();
    
    //find the basins
    for c in coords{
        let size = iter_search(c[0],c[1],&v).len();
        sorted_insert(&mut basin_sizes, size);
    }
    println!("Part two answer: {} x {} x {} = {}", basin_sizes[basin_sizes.len()-3],basin_sizes[basin_sizes.len()-2],basin_sizes[basin_sizes.len()-1],
    basin_sizes[basin_sizes.len()-3]*basin_sizes[basin_sizes.len()-2]*basin_sizes[basin_sizes.len()-1]);
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

fn iter_search(x:usize, y:usize, map: &Vec<Vec<usize>>) ->  HashMap<String, usize>{
    let mut unique: HashMap<String,usize > = HashMap::new();//coords formatted as "x,y"
    unique.insert( String::from(format!("{},{}",x,y)),map[x][y]);
    
    let mut prev_length = unique.len();

    loop{
        let temp = unique.clone();
        for (key,_value) in temp.iter(){
            let neighbors = get_neighbors(key, &map);
            for n in neighbors{
                unique.insert( String::from(format!("{},{}",n[0],n[1] )),map[n[0]][n[1]]);
                //unique.insert(String::from(""),1);
            }
        }

        
        if unique.len() == prev_length{
            break;
        }else{
            prev_length = unique.len();
        }
    }
    unique
}

fn get_neighbors(key: &str, map:&Vec<Vec<usize>> ) -> Vec<Vec<usize>>{
    let coords: Vec<usize> = key.split(",").collect::<Vec<&str>>().into_iter().map(|s| s.parse::<usize>().unwrap() ).collect();
    let y = coords[0];
    let x = coords[1];
    let mut valid: Vec<Vec<usize>> = Vec::new();
    match y{
        0 => (),
        _=> if map[y-1][x] != 9 { valid.push( vec!(y-1,x)  );  }
    };
    match x{
        0 => (),
        _=> if map[y][x-1] != 9 { valid.push( vec!(y,x-1)  );  }
    };
    
    
    
    if x != map[0].len()-1{
        if map[y][x+1] != 9{
            valid.push(vec!( y,x+1  ));
        }
    }

    if y != map.len()-1{
        if map[y+1][x] != 9{
            valid.push(vec!( y+1,x  ));
        }
    }
    
    valid
}


//read the input data, splitting into Display objects
fn process_file(filename: &str) -> Vec<Vec<usize>>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut v : Vec<Vec<usize>> = Vec::new();
    //should have one line per display
    for l in lines{
        let temp:Vec<&str> = l.split("").collect();
        
        let x:Vec<usize> = temp[1..temp.len()-1].into_iter().map( |x| x.parse::<usize>().unwrap()  ).collect();
        v.push(x);
    }
    v
}