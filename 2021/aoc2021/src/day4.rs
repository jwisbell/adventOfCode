use std::fs;
use std::collections::HashMap;



pub fn main(){
    //let filename = "./inputs/day4a_test.txt";
    let filename = "./inputs/day4.txt";
    let (mut boards, rolls) = process_file(filename);

    println!("{:?}",rolls);
    //println!("{:?}",boards[0].values.get(&99)  );


    //part_one(&mut boards, &rolls);
    part_two(&mut boards, &rolls);
}


fn part_one(boards: &mut Vec<Board>, rolls: &Vec<usize>){

    for (i,r) in rolls.iter().enumerate() {
        let mut won = false;
        for b in &mut *boards{
            b.mark_value(*r);
            if i > 4{
                if b.check_winner(){
                    won = true;
                    println!("Part one answer is {}x{}={}",b.score, r, b.score*r);
                    break;
                }
            }
        }
        if won{
            break;
        }
            
    }
}

fn part_two(boards: &mut Vec<Board>, rolls: &Vec<usize>){

    let mut indices: Vec<usize> = Vec::new();
    for i in 0..boards.len(){
        indices.push(i);
    }

    for (i,r) in rolls.iter().enumerate() {
        
        let mut temp_indices: Vec<usize> = Vec::new();

        for j in &indices{
            let b = &mut boards[*j];
            b.mark_value(*r);
            
            if !b.check_winner(){
                temp_indices.push(*j);
            }else{
                println!("{} won!",*j);
            }      
        }

        if temp_indices.len() < 1{
            //no more boards, so we've reached the last one
            println!("{:?}",temp_indices);
            println!("{:?}",r);

            println!("Part two answer is {}x{}={}", boards[indices[0]].score,r,(boards[indices[0]].score)*r );
            break;
        }else{
            indices = temp_indices;
        }   
    }
}


struct Board{
    values: HashMap<usize, bool>,
    locations: Vec<usize>,
    score: usize,
}

impl Board{
    fn mark_value(&mut self, x:usize){
        let result = self.values.get(&x);
        match result{
            None => {},
            _ => {  *self.values.get_mut(&x).unwrap() = true;
                    self.score -= x;},
        }
    }

    fn check_winner(&self) -> bool{
        for i in (0..25).step_by(5){
            
            let mut okay:bool = true; 
            for j in i..i+5{
                okay = okay && *self.values.get(&self.locations[j]).unwrap();
                if !okay{break;}
            }
            if okay{
                return true;
            }
        }
        
        for i in 0..5{
            let mut okay:bool = true; 
            for j in (i..25).step_by(5)   {
                okay = okay && *self.values.get(&self.locations[j]).unwrap();
                if !okay{break;}
            }
            if okay{
                return true;
            }
        }

        let spacing = 6;
        let diag1 =    *self.values.get(&self.locations[0*spacing]).unwrap() 
                    && *self.values.get(&self.locations[1*spacing]).unwrap() 
                    && *self.values.get(&self.locations[2*spacing]).unwrap()
                    && *self.values.get(&self.locations[3*spacing]).unwrap()
                    && *self.values.get(&self.locations[4*spacing]).unwrap();

        let spacing = 4;
        let diag2 =    *self.values.get(&self.locations[spacing]).unwrap() 
                    && *self.values.get(&self.locations[spacing*2]).unwrap() 
                    && *self.values.get(&self.locations[spacing*3]).unwrap()
                    && *self.values.get(&self.locations[spacing*4]).unwrap()
                    && *self.values.get(&self.locations[spacing*5]).unwrap();
        diag1 || diag2
    }

    

}

//modified from yesterday to return the vec of int arrays
fn process_file(filename: &str) -> (Vec<Board>, Vec<usize> ){
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    //the rolls are in the first line
    let temp:Vec<&str> = lines[0].split(",").collect(); 
    let mut rolls = Vec::new();
    for t in temp{
        rolls.push(  t.trim().parse::<usize>().unwrap());
    }


    let mut v : Vec<Board> = Vec::new();
    for i in (2..lines.len()).step_by(6)  {
        
        let mut board = Board{
            values: HashMap::new(),
            locations: Vec::new(),
            score:0,
        };

        for row in &lines[i..i+5 ]{
            let temp:Vec<&str> = row.split_whitespace().collect();

            for t in temp{
                let x = t.trim().parse::<usize>().unwrap();
                board.values.insert(x , false );
                board.locations.push(x);
                board.score += x;
            }
        }
        v.push(board);
    }
    (v, rolls)
}

