use std::fs;
use std::collections::HashMap;

pub fn main(){
    //let filename = "./inputs/day8_test.txt";
    let filename = "./inputs/day8.txt";
    let mut displays = process_file(filename);

    part_one(&mut displays);
    part_two(&mut displays);
}

fn part_one(v:&mut Vec<Display>){
    let mut total = 0;
    for display in v{
        total += display.unique_outputs();
        let test = display.compute_mode( &display.inputs[..].to_vec(),"a");
    }
    println!("Part 1 answer: {}",total);
}

fn part_two(v:&mut Vec<Display>)
{
    let mut total = 0;
    for display in v{
        total += display.deduce();
    }
    println!("Part 2 answer: {}",total);
}


#[derive(Debug)]
struct Display{
    inputs: Vec<String>,
    outputs: Vec<String>,
    codes: Vec<String>,
}

impl Display{
    //count unique outputs for part 1
    fn unique_outputs(&mut self) -> usize{
        let mut count = 0;
        for o in &self.outputs{
            match o.len(){
                2 => { //1
                    count += 1;
                    self.codes[1] = o.to_string();},
                4 => { //4
                    count += 1;
                    self.codes[4] = o.to_string();},
                3 => { //7
                    count += 1;
                    self.codes[7] = o.to_string();},
                7 => { //8
                    count += 1;
                    self.codes[8] = o.to_string();},
                _ => {},
            }
        }
        count
    }

    //compute the mode character
    fn compute_mode(&self, sub_vec: &Vec<String>, ignore:&str ) -> String{
        let mut counts:HashMap<&str, usize> =  HashMap::new();
        for s in sub_vec{
            let temp:Vec<&str> = s.split("").collect();
            for i in 0..temp.len(){
                
                if !ignore.contains(temp[i]){
                    let result = counts.get(temp[i]);
                    match result{
                        None => { counts.insert(temp[i], 0);  },
                        _ =>    {  
                                    *counts.get_mut(temp[i] ).unwrap() += 1;
                                },
                    }
                }
            }
        }
        //iterate over and find max
        let mut max_val = 0;
        let mut max_char = "";
        for (key,val) in counts.iter(){
            if val > &max_val{
                max_char = key;
                max_val = *val;
            }
        }
        //note this is dangerous in cases with ties
        String::from(max_char)
    }

    //for these inputs, decode the values and return the output as int
    fn deduce(&self) -> usize{
        //we know 1,4,7,8 and can figure out others with calculating 
        let mut mapping:HashMap<&str, &str> =  HashMap::new();

        //first id the segments with 5 chars (2,3,5)
        //then those with 6 chars (6,9,0)
        let mut five_indices: Vec<usize> = Vec::new();
        let mut six_indices: Vec<usize> = Vec::new();
        for (i,input) in self.inputs.iter().enumerate(){
            if(input.len() == 5){
                five_indices.push(i);
            }else if(input.len() == 6){
                six_indices.push(i);
            }else if(input.len() == 2){
                mapping.insert("1",input);
            }else if(input.len() == 4){
                mapping.insert("4",input);
            }else if(input.len() == 3){
                mapping.insert("7",input);
            }else if(input.len() == 7){
                mapping.insert("8",input);
            }
            
        }

        //6 only has one component from 1 (bottom right) -> top right
        //check if (6,9,0) contain 1 or 2 chars from 1
        for i in &six_indices{
            let temp = mapping.get("1").unwrap();
            if self.inputs[*i as usize].contains(&temp[..1]) ^ self.inputs[*i as usize].contains(&temp[1..])
            {
                mapping.insert("6", &self.inputs[*i as usize]);
                break;
            } 
        }

        //0 has no center value (mode of all 5-digit chars and 4)
        let mut temp: Vec<String> = Vec::new();
        for i in &five_indices{
            temp.push(  (&self.inputs[*i as usize][..]).to_string()  );
        }
        temp.push( mapping.get("4").unwrap().to_string()  );

        let center = self.compute_mode(&temp,"" );
        for i in &six_indices{
            if !self.inputs[*i as usize].contains(&center) {
                mapping.insert("0", &self.inputs[*i as usize]);
                break;
            } 
        }
        
        
        //9 is remaining 6-char
        for i in &six_indices{
            if self.inputs[*i as usize] != mapping.get("6").unwrap().to_string() 
                && self.inputs[*i as usize] != mapping.get("0").unwrap().to_string(){
                mapping.insert("9", &self.inputs[*i as usize]);
                break;
            }
        }

        //2 doesn't have this bottom right
        let bottom_right = self.compute_mode( &vec![mapping.get("1").unwrap().to_string(), mapping.get("6").unwrap().to_string()],""   );
        for i in &five_indices{
            if !self.inputs[*i as usize].contains(&bottom_right)
            {
                mapping.insert("2", &self.inputs[*i as usize]);
                break;
            } 
        }

        //3 has all of 1 in it
        for i in &five_indices{
            let temp = mapping.get("1").unwrap();
            if self.inputs[*i as usize].contains(&temp[..1]) && self.inputs[*i as usize].contains(&temp[1..])
            {
                mapping.insert("3", &self.inputs[*i as usize]);
                break;
            } 
        }

        //5 is last
        for i in &five_indices{
            if self.inputs[*i as usize] != mapping.get("2").unwrap().to_string() 
                && self.inputs[*i as usize] != mapping.get("3").unwrap().to_string(){
                mapping.insert("5", &self.inputs[*i as usize]);
                break;
            }
        }

        //format final number from outputs
        //first invert hashmap
        let mut mapping_inv:HashMap<&str, &str> =  HashMap::new();
        for (key,value) in mapping.iter(){
            mapping_inv.insert(value, key);
        }

        //convert to int and return it
        let value = format!("{}{}{}{}",
            &mapping_inv.get(&*self.outputs[0]).unwrap(),
            &mapping_inv.get(&*self.outputs[1]).unwrap(),
            &mapping_inv.get(&*self.outputs[2]).unwrap(),
            &mapping_inv.get(&*self.outputs[3]).unwrap()
            ).parse::<usize>().unwrap();
        value
    }
}

//sort the characters in a string
fn sort_str(s: &str) -> String{
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars.iter().rev())
}


//read the input data, splitting into Display objects
fn process_file(filename: &str) -> Vec<Display>{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut v : Vec<Display> = Vec::new();
    //should have one line per display
    for l in lines{
        let temp:Vec<&str> = l.split("|").collect();
        let d = Display{
            inputs:  temp[0].trim().split_whitespace().into_iter().map(|s| sort_str(s)).collect(),
            outputs: temp[1].trim().split_whitespace().into_iter().map(|s| sort_str(s)).collect(),
            codes: vec![String::from("");10],
        };
        v.push(d);
    }
    
    v
}