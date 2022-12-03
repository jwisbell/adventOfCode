use std::fs;
use std::collections::HashMap;



pub fn main(){
    //let filename = "./inputs/day13_test.txt";
    let filename = "./inputs/day13.txt";
    
    let mut dots = process_file(filename);
    
    part_one(&mut dots);
    part_two(&mut dots);
}


fn part_one(dots: &mut Dots){

    //just do the first fold
    if dots.folds[0].is_y{
        dots.do_foldy(dots.folds[0].location);
    }else{
        dots.do_foldx(dots.folds[0].location);
    }
    
    println!("Part one: {}", dots.coords.len());

}

fn part_two(dots: &mut Dots){

    //now do all the folds and print the output
    for i in 0..dots.folds.len() {
        if dots.folds[i].is_y{
            dots.do_foldy( dots.folds[i].location);
        }else{
            dots.do_foldx(dots.folds[i].location);
        }
    }
    
    
    println!("Part two: ");
    dots.print();

}


#[derive(Debug)]
struct Dots{
    coords: HashMap<String, bool>,
    folds: Vec<Fold>,
    size: Vec<isize>,
}

#[derive(Debug)]
struct Fold{
    is_y: bool,
    location: isize,
}


impl Dots{
    fn do_foldy(&mut self, fold_loc:isize){
        let mut new_map: HashMap<String, bool> = HashMap::new();
        let mut max_y:isize = 0;
        for (key, _value) in self.coords.iter(){
            let coords: Vec<isize> = key.split(",").collect::<Vec<&str>>().into_iter().map(|s| s.parse::<isize>().unwrap() ).collect();
            let x = coords[0];
            let mut y = coords[1];
            if y > fold_loc{
                y = -1*(y-fold_loc) + fold_loc;
            }
            if y > max_y{max_y = y;}
            new_map.insert( String::from(format!("{},{}",x,y )), true );
        }
        self.coords = new_map;
        self.size[1] = max_y; 
    }
    
    fn do_foldx(&mut self, fold_loc:isize){
        let mut new_map: HashMap<String, bool> = HashMap::new();
        let mut max_x:isize = 0;
        for (key, _value) in self.coords.iter(){
            let coords: Vec<isize> = key.split(",").collect::<Vec<&str>>().into_iter().map(|s| s.parse::<isize>().unwrap() ).collect();
            let mut x = coords[0];
            let  y = coords[1];
            
            if x > fold_loc{
                x = -1*(x-fold_loc) + fold_loc;
            }
            if x > max_x { max_x = x;}
            new_map.insert( String::from(format!("{},{}",x,y )), true );
        }
        self.coords = new_map;
        self.size[0] = max_x;
    }

    //format the array into printable text instead of coords
    fn print(&self){
        let mut displ: Vec<String> = Vec::new();
        for i in 0..self.size[1]+1{
            let mut temp: String = String::new();
            for j in 0..self.size[0]+1{
                temp.push( '_' );
            }
            displ.push(  temp   );
        }

        println!("{:?}", displ.len());
        for (key, _value) in self.coords.iter(){
            let coords: Vec<isize> = key.split(",").collect::<Vec<&str>>().into_iter().map(|s| s.parse::<isize>().unwrap() ).collect();
            let  x = coords[0];
            let  y = coords[1];
            displ[y as usize].replace_range((x as usize)..(x as usize)+1,"#"  );
        }
        for s in displ{
            println!("{}",s);
        }
    }
}



//read the input data, splitting into Display objects
fn process_file(filename: &str) -> Dots{

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut d = Dots{
        coords: HashMap::new(),
        folds: Vec::new(),
        size: Vec::new(),
    };
    d.size.push(0);
    d.size.push(0);

    //should have one line per display
    for l in 0..lines.len(){
        let temp:Vec<&str> = lines[l].split(",").collect();
        if temp.len() ==2{
            println!("{:?}",temp);
            d.coords.insert(String::from(format!("{},{}",temp[0].trim(),temp[1].trim() )), true );
        }else{
            let temp:Vec<&str> = lines[l].split("=").collect();
            if temp.len() == 2{
                let f = Fold{
                    is_y: temp[0].contains('y'),
                    location:  temp[1].trim().parse::<isize>().unwrap(),
                };
                d.folds.push(f);
            }
        }
        
    }

    

    d
}