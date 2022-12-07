use std::fs;

pub fn main(){
    //let filename = "./inputs/day4_test.txt";
    let filename = "./inputs/day4.txt";
    let assignments = process_input(filename);

    part_one(&assignments);
    part_two(&assignments);

}

fn part_one(assignments: &Vec<String>){
    let assignments: Vec<&str> = assignments.iter().map(|s| &**s).collect();

    //for each line, check whether one range is entirely inside of the other
    let mut n_contained = 0;

    for a in assignments{
        let ranges: Vec<&str> = a.split(',').collect();
        let mut r1  = Range{minval:0, maxval:0};
        let mut r2 =  Range{minval:0, maxval:0};
        r1.init(&ranges[0]);
        r2.init(&ranges[1]);
        let test = r1.check_inside(&r2) || r2.check_inside(&r1);
        if test{
            n_contained += 1;
        }
        //println!("{:?} {:?} {}", r1, r2, test);
    }

    println!("The number of contained ranges for part 1 is {}", n_contained);
}

fn part_two(assignments: &Vec<String>){
    let assignments: Vec<&str> = assignments.iter().map(|s| &**s).collect();

    //for each line, check whether one range is entirely inside of the other
    let mut n_contained = 0;

    for a in assignments{
        let ranges: Vec<&str> = a.split(',').collect();
        let mut r1  = Range{minval:0, maxval:0};
        let mut r2 =  Range{minval:0, maxval:0};
        r1.init(&ranges[0]);
        r2.init(&ranges[1]);
        let test = r1.check_overlap(&r2) || r2.check_overlap(&r1);
        if test{
            n_contained += 1;
        }
        //println!("{:?} {:?} {}", r1, r2, test);
    }

    println!("The number of overlapped ranges for part 2 is {}", n_contained);
}

#[derive(Debug)]
struct Range{
    minval: usize,
    maxval: usize
}

impl Range{
    fn check_inside(&self, other:&Range) -> bool{
        if other.minval >= self.minval && other.maxval <= self.maxval{
            return true
        }
        false
    }

    fn check_overlap(&self, other:&Range) -> bool{
        if other.minval >= self.minval && other.minval <= self.maxval{
            return true
        }

        false
    }


    fn init(&mut self, s:&str){
        let nums: Vec<&str> = s.split('-').collect();
        let x = nums[0].parse::<usize>().unwrap();
        let y = nums[1].parse::<usize>().unwrap();
        if x < y{
            self.minval = x;
            self.maxval = y;
        }else{
            self.minval = y;
            self.maxval = x;
        }
    }
}


//return the lines from the input file as a vector
fn process_input(filename:&str) -> Vec<String>{
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut return_vals: Vec<String> = Vec::new();
    
    for l in lines{
        return_vals.push( String::from(l) );
    }

    let return_vals = return_vals.clone();
    return_vals
}