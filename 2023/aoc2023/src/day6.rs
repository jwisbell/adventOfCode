use std::fs;

pub fn main(){
    //let filename = "./inputs/day6_test.txt";
    let filename = "./inputs/day6.txt";
    let races = process_input(filename);    
    
    println!("{:?}", races);  
    part1(races.clone());
    part2(races.clone());
    
}

fn part1(races:Vec<Race>){
    //for each race, calculate the number of ways you can win
    //multiply all of these together
    let mut ways:Vec<usize> = Vec::new();
    assert_eq!(find_ways(7, 9), 4); //unit test for the example
    for race in races{
        ways.push(find_ways(race.time, race.distance));
    }

    
    println!("Part 1");
    let mut product = 1;
    let _temp:Vec<usize> = ways.iter().map(|&x| {product = product*x; x}).collect();
    println!("The product of all ways is {:?}", product);
}

fn find_ways(time:usize, distance:usize)->usize{
    //for a given time and distance, count the number of ways
    let mut ways:usize = 0;
    for i in 0..(time+1){
        let drive_time = time - i;
        let d = drive_time*i; //i is the speed
        if d>distance{
            ways += 1;
        }
    }

    ways
}


fn part2(races:Vec<Race>){
    //now find the number of possibilities for one longer race...
    println!("Part 2");
    let time = String::from_iter(races.iter().map(|x| x.time).collect::<Vec<usize>>().iter().map(|x|x.to_string()).collect::<Vec<String>>()).parse::<usize>().unwrap();
    let distance = String::from_iter(races.iter().map(|x| x.distance).collect::<Vec<usize>>().iter().map(|x|x.to_string()).collect::<Vec<String>>()).parse::<usize>().unwrap();
    println!("{} {}", time, distance);
    println!("{}", find_ways(time, distance));
}

#[derive(Debug, Clone)]
struct Race{
    time: usize,
    distance:usize
}


fn process_input(filename:&str)->Vec<Race>{
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    

    let mut races:Vec<Race> = Vec::new();
    
    let times:Vec<usize> = lines[0].trim().split(":").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<usize>().unwrap()).collect();
    let distances:Vec<usize> = lines[1].trim().split(":").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<usize>().unwrap()).collect();

    for i in 0..times.len(){
        races.push(Race { time: times[i], distance: distances[i] })
    }
    races
}