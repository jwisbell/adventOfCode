use std::fs;

//need to change for test and real data
const GRIDSIZE:usize = 1000;

pub fn main(){
    //let filename = "./inputs/day5_test.txt";
    let filename = "./inputs/day5.txt";
    
    //all the magic happens in the formatting of the lines object here
    let lines = process_file(filename,false);
    part_one(&lines);

    //remake the lines object with diagonals enabled
    let lines = process_file(filename,true);
    part_two(&lines);
}

//simply iterate over all lines, and add them to the map
//print number of >=2 crossings
fn part_one(lines: &Vec<Line>){
    let mut m = Map{
        mymap: vec![ vec![0;GRIDSIZE];GRIDSIZE     ],
        n_gtr2: 0,
    };

    for l in lines{
        m.add_line( l );
    }

    println!("Part one answer: {}", m.n_gtr2);
}

//same as part 1 but with diagonals
fn part_two(lines: &Vec<Line>){
    let mut m = Map{
        mymap: vec![ vec![0;GRIDSIZE];GRIDSIZE     ],
        n_gtr2: 0,
    };

    for l in lines{
        m.add_line( l );
    }
    println!("Part two answer: {}", m.n_gtr2);
}

#[derive(Debug)]
//collect the line coords, 
//more memory efficient to do (x0,y0), mag, direction for longer lines, but hey
struct Line{
    xvals: Vec<usize>,
    yvals: Vec<usize>,
}



//struct to hold grid
struct Map{
    //had to make 2d vec instead of array due to stack overflow
    mymap: Vec< Vec<usize> >,
    n_gtr2: usize, //crossing count
}

impl Map{
    fn add_line(&mut self, line: &Line ){
        for i in 0..line.xvals.len(){
            self.mymap[line.yvals[i]  ][line.xvals[i]] += 1;

            //only add one to count if this point is just now at 2 crossings
            if self.mymap[line.yvals[i]][line.xvals[i]] == 2{
                self.n_gtr2 += 1;
            }
        }
    }
}



//modified from yesterday to return the vec of line arrays
fn process_file(filename: &str, use_diag:bool) -> Vec<Line>{
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut v : Vec<Line> = Vec::new();

    for i in 0..lines.len() {
        //line object
        let mut temp_line = Line{
            xvals: Vec::new(),
            yvals: Vec::new(),
        };

        //split into start and stop
        let temp: Vec<&str> = lines[i].split("->").collect();
        println!("{:?}", temp);

        //for part1 only consider vert and horizontal lines
        //get start/end coords here from the string
        let temp1: Vec<&str> = temp[0].split(",").collect();
        let x1 = temp1[0].trim().parse::<usize>().unwrap();
        let y1 = temp1[1].trim().parse::<usize>().unwrap();

        let temp2: Vec<&str> = temp[1].split(",").collect();
        let x2 = temp2[0].trim().parse::<usize>().unwrap();
        let y2 = temp2[1].trim().parse::<usize>().unwrap();

        //vertical line
        if x1==x2{
            if y1<y2{
                for y in y1..y2+1{
                    temp_line.yvals.push(y);
                    temp_line.xvals.push(x1);
                }
            }else{
                for y in y2..y1+1{
                    temp_line.yvals.push(y);
                    temp_line.xvals.push(x1);
                }
            }
            
            v.push(temp_line);
        }else if y1==y2{ //horiz line
            if x1<x2{
                for x in x1..x2+1{
                    temp_line.yvals.push(y1);
                    temp_line.xvals.push(x);
                }
            }else{
                for x in x2..x1+1{
                    temp_line.yvals.push(y1);
                    temp_line.xvals.push(x);
                }
            }
            
            v.push(temp_line);
        }else{ // diagonal lines at 45 deg
            if y1 > y2 && x1 > x2{ //down left
                
                for j in 0..y1-y2+1 {
                    temp_line.yvals.push(y2+j);
                    temp_line.xvals.push(x2+j);
                } 
            }else if y1 < y2 && x1 > x2{ //up left
                
                for j in 0..y2-y1+1 {
                    temp_line.yvals.push(y1+j);
                    temp_line.xvals.push(x1-j);
                } 
            }else if y1 < y2 && x1 < x2{ //up right
                for j in 0..y2-y1+1 {
                    temp_line.yvals.push(y1+j);
                    temp_line.xvals.push(x1+j);
                } 
            }else if y1 > y2 && x1 < x2{ //down right
                for j in 0..y1-y2+1 {
                    temp_line.yvals.push(y2+j);
                    temp_line.xvals.push(x2-j);
                } 
            }
            if use_diag{
                v.push( temp_line ); 
            }
        }
    }
    v
}