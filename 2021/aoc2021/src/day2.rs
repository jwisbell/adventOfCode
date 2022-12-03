use std::fs;

pub fn main(){
    //let filename = "./inputs/day2_test.txt";
    let filename = "./inputs/day2a.txt";
    let commands = process_file(filename);
    
    part_one(&commands);
    part_two(&commands);

}

fn part_one(commands: &Vec<Command>){
    let mut pos = Position{
        horiz:0,
        depth:0,
        aim:0,
    };
    //simply iterate over commands and do the correct transformation
    for cmd in commands{
        //need to convert to &str for string literal comparison
        match  &cmd.direction[..]{
            "forward" => {  pos.horiz = pos.horiz + cmd.distance;},
            "up"=>       {  pos.depth = pos.depth - cmd.distance;},
            "down"=>     {  pos.depth = pos.depth + cmd.distance;},
            _ => {println!("Invalid input");},
        }
    }

    println!("The value for part 1 is {}x{}={}",pos.horiz,pos.depth, pos.horiz*pos.depth);
}

fn part_two(commands: &Vec<Command>){
    let mut pos = Position{
        horiz:0,
        depth:0,
        aim:0,
    };
    //simply iterate over commands and do the correct transformation
    for cmd in commands{
        //need to convert to &str for string literal comparison
        match  &cmd.direction[..]{
            "forward" => {  pos.horiz = pos.horiz + cmd.distance;
                            pos.depth = pos.depth + pos.aim * cmd.distance;},
            "up"=>       {  pos.aim = pos.aim - cmd.distance;},
            "down"=>     {  pos.aim = pos.aim + cmd.distance;},
            _ => {println!("Invalid input");},
        }
    }
    println!("The value for part 2 is {}x{}={}",pos.horiz,pos.depth, pos.horiz*pos.depth);
}

//structs for convenience
struct Position{
    horiz: isize,
    depth: isize,
    aim: isize,
}

#[derive(Debug)]
struct Command{
    direction:String,
    distance: isize,
}

//modified from yesterday to return the vec of Command structs
fn process_file(filename: &str) -> Vec<Command>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    //each line should be formatted as
    // forward 5
    // up 3
    // down 4
    //etc.
    let mut v : Vec<Command> = Vec::new();
    for l in lines{
        //split the line into direction and distance
        let temp:Vec<&str> = l.split(" ").collect();
        let cmd = Command{
            direction: temp[0].trim().to_string(),
            distance: temp[1].trim().parse::<isize>().unwrap(),
        };
        v.push(cmd);
    }
    v
}