use std::fs;

pub fn main(){
    //let filename = "./inputs/day1_test.txt";
    let filename = "./inputs/day1.txt";
    let calories = process_input(filename);

    part_one(&calories);
    part_two(&calories);

}

fn part_one(calories: &Vec<usize>){
    //need to find the maximum value
    let mut index = 0;
    let mut maxval = calories[0];

    for i in 0..calories.len(){
        if calories[i] > maxval{
            maxval = calories[i];
            index = i;
        }
    }

    println!("The elf with most calories is elf {} with {} calories", index, maxval);
}

fn part_two(calories: &Vec<usize>){
    //need to find the maximum 3 values
    //this means i need to sort! This also serves as a solution to part 1
    //implementing bubble sort 
    let bubble = bubble_sort(calories);
    
    println!("The 3 elves with most calories have {} total calories", bubble[bubble.len()-1] + bubble[bubble.len()-2] + bubble[bubble.len()-3]);

    
}

fn bubble_sort(values: &Vec<usize>) -> Vec<usize>{
    let mut sorted: Vec<usize> = values.clone();
    let mut swapped = true;

    //continue until there are no more swapped elements
    while swapped{
        swapped = false;
        for i in 0..values.len()-1{
            let j = i + 1;
            let a = sorted[i];
            let b = sorted[j];
            if a > b {
                sorted[i] = b;
                sorted[j] = a; 
                swapped = true;
            }
        }
    }
    sorted
}




fn process_input(filename:&str) -> Vec<usize>{
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut cal_values: Vec<usize> = Vec::new();
    let mut temp_sum = 0;

    //i only need the total calories for each elf, so just store and reset the calorie sum whenever there is a newline
    for l in &lines{
        if l.trim().is_empty(){
            cal_values.push( temp_sum );
            temp_sum = 0;
        }else{
            let value = l.parse::<usize>().unwrap();
            temp_sum += value;
        }

    }

    cal_values
}