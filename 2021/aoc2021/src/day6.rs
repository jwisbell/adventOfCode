use std::fs;

pub fn main(){
    //let filename = "./inputs/day6_test.txt";
    let filename = "./inputs/day6.txt";
    let mut fish_vec = process_file(filename);

    
    //part_one(&mut fish_vec);
    part_two(&fish_vec);
}

fn part_one(v: &mut Vec<Fish>){
    //naive implementation that takes exponentially long...
    let n_days:usize = 80;
    for i in 0..n_days{
        let start_len = v.len();
        for k in 0..start_len{
            let mut f = &mut v[k];
            let spawn = f.update();
            if spawn{
                let temp_fish = Fish{
                    timer: (f.timer + 2),
                };
                v.push(temp_fish);
            }
        }
    }

    println!("Afer {} days there are {} fish",n_days, v.len());

}

fn part_two(v:&Vec<Fish>){
    //part 2 punishes you for doing what I did in part 1
    //now I want to go in steps of 0-6, and "school" the fish
    //ie have an array where each element is num fish on that day to spawn
    //should only be O(n) where n is n_days

    //first group the fish by timer value
    let mut init_school = [0;7];
    for fish in v{
        init_school[fish.timer] += 1;
    }
    
    //set the length of time
    let n_days = 256;
    let mut days = 0;
    //array to store upcoming fish cycles
    let mut next_school = [0;9];
    while days < n_days{
        for i in 0..7{
            let n_fish = init_school[i];
            next_school[i+2] += n_fish;
            days += 1;
            if days>=n_days{
                break;
            }
        }
        if days < n_days{
            //school the fish
            for i in 0..7{
                init_school[i] += next_school[i];
                next_school[i] = 0;
            }
            for i in 7..next_school.len(){
                next_school[i-7] = next_school[i];
                next_school[i] = 0; 
            }
        }
    }
    println!("There are {} fish on day {}", sum_arr(&init_school)+sum_arr(&next_school),n_days );
}

fn sum_arr(x:&[usize] ) ->usize{
    let mut sum = 0;
    for y in x{
        sum += y;
    }
    sum
}


#[derive(Debug)]
struct Fish{
    timer:usize,
}

impl Fish{
    fn update(&mut self) -> bool{
        if self.timer == 0{
            self.timer = 6;
            return true;
        }else{
            self.timer -=1;
        }
        false
    }
}

//modified from yesterday to return the vec of Command structs
fn process_file(filename: &str) -> Vec<Fish>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    //should have one line which is initial state
    let mut v : Vec<Fish> = Vec::new();
    let temp:Vec<&str> = lines[0].split(",").collect();

    for t in temp{
        let temp_fish = Fish{
            timer: t.trim().parse::<usize>().unwrap(),
        };
        v.push(temp_fish);
    }
    v
}