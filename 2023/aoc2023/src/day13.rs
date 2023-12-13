use std::fs;

pub fn main(){
    //let filename = "./inputs/day13_test.txt";
    let filename = "./inputs/day13.txt";
    let inputs = process_input(filename);
    
    part1(inputs.clone());
    part2(inputs.clone());

    
}

fn part1(data:Vec<Pattern>){
    //find the location of each vertical or horizontal reflection
    println!("Part 1");
    let mut count = 0;
    for pattern in data{

        let (horiz_idx,_) = find_reflection(&pattern.rows,1);

        let (vert_idx, _) = find_reflection(&pattern.columns,1);

        if horiz_idx != pattern.rows.len()+1{
            //found horizontal
            count += 100*horiz_idx;
        }else if vert_idx != pattern.columns.len()+1{
            //found vertical
            count += vert_idx
        }else{
            //something went wrong...
        }

    }
    println!("The summary is {}", count);
}


fn part2(data:Vec<Pattern>){
    //find the location of the smudge 
    println!("Part 2");
    let mut count = 0;
    for i in 0..data.len(){
        let pattern = &data[i];
        let (horiz_idx, horiz_smudge) = find_reflection(&pattern.rows,2);
        let (vert_idx, vert_smudge) = find_reflection(&pattern.columns,2);
        if horiz_idx != pattern.rows.len()+1 && horiz_smudge{
            //found horizontal
            count += 100*horiz_idx;
        }else if vert_idx != pattern.columns.len()+1 && vert_smudge{
            //found vertical
            count += vert_idx
        }else{
            //something went wrong...
            println!("{:?} {}", pattern, i);
        }
    }

    println!("The summary is {}", count);
}



#[derive(Debug, Clone)]
struct Pattern{
    rows: Vec<Vec<usize>>,
    columns: Vec<Vec<usize>>
}

fn find_reflection(arr:&Vec<Vec<usize>>, part:usize)->(usize,bool){
    //find the index of the row where a reflection happens
    //this works for both rows and columns if the columns 2d array has been transposed
    //checks adjacent rows to ensure reflection
    let mut prev_row = &arr[0];
    let mut smudged = false;
    println!("{} {}", arr.len(), arr[0].len());
    for idx in 1..arr.len(){
        let current_row = &arr[idx];
        if current_row == prev_row{
            //we have possibly found a reflection
            let valid_reflection = check_reflection(idx, arr);
            let (smudged_reflection, smudged_idx) = check_smudged_reflection(idx, arr);

            if part==1 && valid_reflection{
                return (idx, false);
            }else if part==2 && smudged_reflection && smudged_idx!=current_row.len()+1{
                return (idx, true);
            }
        }else if part == 2{
            //the rows might match if there is a smudge
            let diff = vec_subtract(current_row,prev_row);
            
            let mut still_valid = true;
            let mut smudge_idx = diff.len()+1;
            smudged = false;
            
            for subidx in 0..diff.len(){
                if (diff[subidx] == 1 || diff[subidx] == -1) && still_valid{
                    smudge_idx = subidx;
                    still_valid = false;
                    smudged = true;
                }else if !still_valid && (diff[subidx] == 1 || diff[subidx] == -1) {
                    smudge_idx = diff.len()+1;
                    smudged = false;
                    break;
                }
            }
            //if there is no valid smudge, this reflection isn't valid anyway
            if smudge_idx != diff.len()+1 && smudged{
                let mut smudged_arr = arr.clone();
                if smudged_arr[idx][smudge_idx] == 1{
                    smudged_arr[idx][smudge_idx] = 0;
                }else{
                    smudged_arr[idx][smudge_idx] = 1;
                }
                if check_reflection(idx, &smudged_arr){
                    return (idx,true);
                }
                
            }
            
        }
        
        prev_row = &arr[idx];
    }
    (arr.len()+1, false)
}

fn check_reflection(idx:usize, arr:&Vec<Vec<usize>>)->bool{
    //check all rows to make sure this isn't just a local reflection
    for i in 1..idx{
        let a = arr.get(idx-i-1);
        let b = arr.get(idx+i);
        //println!("a {:?} b {:?}", a, b);
        if a.is_none() || b.is_none(){
            break;
        }else if Some(a) != Some(b){
            //the valid values dont match
            return false;
        }
    }
    true
}

fn check_smudged_reflection(idx:usize, arr:&Vec<Vec<usize>>)->(bool,usize){
    //check all rows to make sure this isn't just a local reflection
    let mut smudge_idx = arr[0].len()+1;
    for i in 1..idx{
        let a = arr.get(idx-i-1);
        let b = arr.get(idx+i);
        if a.is_none() || b.is_none(){
            break;
        }else if Some(a) != Some(b){
            //the valid values dont match
            //if they differ by just one, then the smudge is possibly there
            let diff = vec_subtract(a.unwrap(), b.unwrap());
            let mut still_valid = true;
            for subidx in 0..diff.len(){
                if (diff[subidx] == 1 || diff[subidx] == -1) && still_valid{
                    still_valid = false;
                    smudge_idx = subidx;
                }else if (diff[subidx] == 1 || diff[subidx] == -1) && !still_valid{
                    smudge_idx = diff.len()+1;
                    break;
                }
            }

            //if there is no valid smudge, this reflection isn't valid anyway
            if smudge_idx == diff.len()+1{
                return (false, smudge_idx);
            }
        }
    }
    (true, smudge_idx)
}

fn vec_subtract(vec1:&Vec<usize>, vec2:&Vec<usize>)->Vec<i32>{
    let mut diff:Vec<i32> = Vec::new();
    assert_eq!(vec1.len(), vec2.len());
    for i in 0..vec1.len(){
        diff.push((vec1[i] as i32) - (vec2[i] as i32) );
    }
    diff
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn process_input(filename:&str) -> Vec<Pattern>{
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    
    let mut maps:Vec<Pattern> = Vec::new();
    let mut buffer:Vec<Vec<usize>> = Vec::new();
    //let mut in_pattern = false;

    for line in lines{
        if line.trim().len()>0{
            let temp:Vec<usize> = line.trim().chars().map(|x| {
                match x{
                    '#'=>1,
                    '.'=>0,
                    _=>0
                }
            }).collect();
            buffer.push(temp);
        }else{
            let p = Pattern{
                rows:buffer.clone(),
                columns:transpose(buffer.clone())
            };
            maps.push(p);
            buffer = Vec::new();
        }
    }
    let p = Pattern{
        rows:buffer.clone(),
        columns:transpose(buffer.clone())
    };
    maps.push(p);

    maps

}
