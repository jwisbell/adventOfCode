use std::fs;

pub fn main(){
    //let filename = "./inputs/day3_test.txt";
    let filename = "./inputs/day3.txt";
    let rows = process_input(filename);    
    
    part1(rows.clone());
    part2(rows);
}

fn part1(arr:Vec<Vec<char>>){
    //loop over each row and find the numbers (separated by . or symbol)
    //then look in adjacent rows for symbols to decide what to keep
    let mut numbers_with_adjacent: Vec<usize> = Vec::new();
    for i in 0..arr.len(){
        let row = &arr[i];
        //temporary values for building a number
        let mut temp_numstr:Vec<char> = Vec::new();
        let mut adjacent_symbol = false;
        let mut in_number = false;
        
        for j in 0..row.len(){
            //find the locations of the digits
            //check adjacent indices for symbols
            if isdigit(row[j]){
                in_number = true;
                if isadjacent(i,j, arr.clone()){
                    adjacent_symbol = true;
                }
                temp_numstr.push(row[j]);
            }else{
                if in_number{
                    println!("Reached end of number {:?} was it near symbol? {}", temp_numstr, adjacent_symbol);
                    if adjacent_symbol{
                        //save the value
                        numbers_with_adjacent.push( String::from_iter(temp_numstr.clone()).parse::<usize>().unwrap() );
                    }
                    
                }
                in_number =false;
                adjacent_symbol =false;
                temp_numstr = Vec::new();
            }

            //catch the edge of row case
            if j == row.len()-1{
                if in_number{
                    println!("Reached end of number {:?} was it near symbol? {}", temp_numstr, adjacent_symbol);
                    if adjacent_symbol{
                        //save the value
                        numbers_with_adjacent.push( String::from_iter(temp_numstr.clone()).parse::<usize>().unwrap() );
                    }
                    
                }
            }
        }
    }

    println!("Part 1");
    println!("The numbers are {:?}", numbers_with_adjacent);
    println!("The sum is  {}", numbers_with_adjacent.iter().sum::<usize>());
}


fn isdigit(c:char)->bool{
    !issymbol(c) && c!='.'
}

fn issymbol(c:char)->bool{
    match c {
        '1'=>false,
        '2'=>false,
        '3'=>false,
        '4'=>false,
        '5'=>false,
        '6'=>false,
        '7'=>false,
        '8'=>false,
        '9'=>false,
        '0'=>false,
        '.' => false,
        _ => true,
    }
}

fn isadjacent(x:usize, y:usize, arr:Vec<Vec<char>>) -> bool{
    //check the adjacent (incl diagonals) characters for a symbol
    let topleft:bool = if x > 0 && y>0{issymbol(arr[x-1][y-1])}else{false};
    let top:bool = if y>0{issymbol(arr[x][y-1])}else{false};
    let topright:bool = if y>0 && x < arr[0].len()-1 {issymbol(arr[x+1][y-1])}else{false};

    let left:bool = if x > 0 {issymbol(arr[x-1][y])}else{false};
    let right:bool = if x < arr[0].len()-1{issymbol(arr[x+1][y])}else{false};

    let bottomleft:bool = if x > 0 && y<arr.len()-1{issymbol(arr[x-1][y+1])}else{false};
    let bottom:bool = if y<arr.len()-1{issymbol(arr[x][y+1])}else{false};
    let bottomright:bool = if y<arr.len()-1 && x < arr[0].len()-1{issymbol(arr[x+1][y+1])}else{false};
    
    return topleft || top || topright || left || right || bottomleft || bottom || bottomright
}

fn reconstruct_number(i:usize,j:usize, arr:Vec<Vec<char>>)->Vec<(usize,usize)>{
    let mut idx:Vec<(usize,usize)> = Vec::new();
    let row = arr[i].clone();

    //first look to the left of the known digit
    let mut temp_idx:Vec<(usize, usize)> = Vec::new();
    for k in (0..j).rev() {
        if isdigit(row[k]){
            temp_idx.push((i,k))
        }else{
            break;
        }
    }
    //add the temp_idx in reverse order to the idx vec
    for x in temp_idx.iter().rev(){
        idx.push(*x);
    }

    //then look to the right
    for k in j..row.len(){
        if isdigit(row[k]){
            idx.push((i,k))
        }else{
            break;
        }
    }

    //return the array of indices
    idx

}

fn part2(arr:Vec<Vec<char>>){
    //for part two, i want to find all gear symbols '*' with two adjacent digits
    //then i will reconstruct the numbers and save

    let mut gear_numbers: Vec<usize> = Vec::new();

    for x in 0..arr.len(){
        for y in 0..arr[x].len(){
            if arr[x][y] == '*'{
                let mut number_idx:Vec<Vec<(usize,usize)>> = Vec::new();
                //we have found a gear, see if the adjacent chars are from two distinct numbers

                //each of these could be done as a function taking the coords as args, 
                //but i left explicit here for clarity, especially with boundary checking
                let topleft:bool = if x > 0 && y>0{isdigit(arr[x-1][y-1])}else{false};
                if topleft{
                    let idx = reconstruct_number(x-1, y-1, arr.clone());
                    if !number_idx.contains(&idx){ //only add it if it's not in the list
                        number_idx.push(idx);
                    }
                }
                
                let top:bool = if y>0{isdigit(arr[x][y-1])}else{false};
                if top{
                    let idx = reconstruct_number(x, y-1, arr.clone());
                    if !number_idx.contains(&idx){
                        number_idx.push(idx);
                    }
                }

                let topright:bool = if y>0 && x < arr[0].len()-1 {isdigit(arr[x+1][y-1])}else{false};
                if topright{
                    let idx = reconstruct_number(x+1, y-1, arr.clone());
                    if !number_idx.contains(&idx){
                        number_idx.push(idx);
                    }
                }

                let left:bool = if x > 0 {isdigit(arr[x-1][y])}else{false};
                if left{
                    let idx = reconstruct_number(x-1, y, arr.clone());
                    if !number_idx.contains(&idx){
                        number_idx.push(idx);
                    }
                }
                let right:bool = if x < arr[0].len()-1{isdigit(arr[x+1][y])}else{false};
                if right{
                    let idx = reconstruct_number(x+1, y, arr.clone());
                    if !number_idx.contains(&idx){
                        number_idx.push(idx);
                    }
                }

                let bottomleft:bool = if x > 0 && y<arr.len()-1{isdigit(arr[x-1][y+1])}else{false};
                if bottomleft{
                    let idx = reconstruct_number(x-1, y+1, arr.clone());
                    if !number_idx.contains(&idx){
                        number_idx.push(idx);
                    }
                }
                let bottom:bool = if y<arr.len()-1{isdigit(arr[x][y+1])}else{false};
                if bottom{
                    let idx = reconstruct_number(x, y+1, arr.clone());
                    if !number_idx.contains(&idx){
                        number_idx.push(idx);
                    }
                }
                let bottomright:bool = if y<arr.len()-1 && x < arr[0].len()-1{isdigit(arr[x+1][y+1])}else{false};
                if bottomright{
                    let idx = reconstruct_number(x+1, y+1, arr.clone());
                    if !number_idx.contains(&idx){
                        number_idx.push(idx);
                    }
                }

                if number_idx.len() == 2{
                    let test = number_idx.iter().map(|entry| {
                        println!("{:?}", entry);
                        String::from_iter(entry.iter().map(|coord| arr[coord.0][coord.1]).collect::<Vec<char>>()).parse::<usize>().unwrap()
                    }).collect::<Vec<usize>>();
                    println!("{},{} has only two matches! {:?} {:?}",x,y, number_idx, test );

                    gear_numbers.push(test[0] * test[1]);
                }
            }
        }
    }

    println!("Part 2");
    println!("The sum is {}", gear_numbers.iter().sum::<usize>());
}



fn process_input(filename:&str)->Vec<Vec<char>>{
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    //save each Game as a vector of N draws
    let mut rows: Vec<Vec<char>> = Vec::new();
    for l in lines{
        let characters:Vec<char> = l.trim().chars().collect();
        rows.push(characters);
    }
    rows
}