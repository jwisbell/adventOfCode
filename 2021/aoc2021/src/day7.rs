use std::fs;

pub fn main(){
    //let filename = "./inputs/day7_test.txt";
    let filename = "./inputs/day7.txt";
    let mut crabs = process_file(filename);

    part_one(&crabs);
    part_two(&crabs);
}

fn part_one(v: &Vec<usize>){
    //use the median position
    println!("Part one cost {} at location {}",
        fuel_sum(v, median(v)) ,median(v)  );
}

fn part_two(v: &Vec<usize>){
    //use the mean position because outliers now have more weight
    let mean = fuel_sum(v,0)/v.len();
    println!("Part two answer {} at location {}", fuel_sum_two(v,mean), mean);
}

//return central value of sorted array
fn median(v: &Vec<usize>) -> usize{
    if v.len()%2 == 1{
        return v[ v.len()/2  ];
    }else{
        return (v[ v.len()/2  ] + v[ v.len()/2-1  ])/2;
    }
    0
}


//for part 1 return sum of vector shifted by position
fn fuel_sum(v: &Vec<usize>, position: usize) -> usize{
    let mut sum = 0;
    for x in v{
        if x > &position{
            sum += x-position;
        }else{
            sum += position - x;
        }
    }
    sum
}

//for part 2 return fuel cost using increased fuel for distance D, fuel = D(D+1)/2
fn fuel_sum_two(v: &Vec<usize>, position: usize) -> usize{
    let mut sum = 0;
    for x in v{
        let mut diff = 0;
        if x > &position{
            diff = x-position
            
        }else{
            diff = position - x;
        }
        let mut temp_sum = diff * (diff+1) / 2;
        
        sum += temp_sum;
    }
    sum
}

//create a sorted list while reading in the data
fn sorted_insert(v: &mut Vec<usize>, x:usize){
    //find the first element that the number is greater than, insert there
    let mut inserted = false;
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
                inserted = true;
                return ();
            }
        }
    }
    // number is greater than other existing ones, add to end
    v.push(x);
    
}

//modified from yesterday to return the vec of Command structs
fn process_file(filename: &str) -> Vec<usize>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    //should have one line which is initial state
    let mut v : Vec<usize> = Vec::new();
    let temp:Vec<&str> = lines[0].split(",").collect();

    for t in temp{
        sorted_insert(&mut v, t.trim().parse::<usize>().unwrap());
    }
    v
}