use std::fs;

pub fn main(){
    //let filename = "./inputs/day5_test.txt";
    let filename = "./inputs/day5.txt";
    let inputs = process_input(filename);    
    
    println!("{:?}", inputs);  
    part1(inputs.clone());
    //part2(inputs.clone());
    part2_improved(inputs.clone());
}

fn part1(data:DataFrame){
    //find the lowest location number
    let mut lowest_location:usize = 9999999999;
    for seed in &data.seeds{
        let loc = get_location(*seed, &data);
        if loc < lowest_location{
            lowest_location = loc;
        }
    }

    println!("Part 1");
    println!("The smallest loc is {}", lowest_location);
}

fn get_location(seed:usize, data:&DataFrame)->usize{
    //find each corresponding index...
    let mut idx = seed;
    for mapping in [&data.seed_to_soil, &data.soil_to_fertilizer, &data.fertilizer_to_water, &data.water_to_light,
         &data.light_to_temperature, &data.temperature_to_humidity, &data.humidity_to_location]{
        
        //println!("Looking for {} in {:?}", idx, mapping[1]);
        let mut range_idx = 0;
        let mut offset = 0;
        let mut found = false;
        for k in 0..mapping[1].len(){
            let range = mapping[1][k];
            if idx >= range.0 && idx < range.1{
                range_idx = k;
                offset = idx - range.0;
                found = true;
                break;
            }
        }
        if found{
            idx = mapping[0][range_idx].0 + offset
        }
        //println!("Continuing with {}", idx);
        
        
    }
    idx
}


fn get_seed(loc:usize, data:&DataFrame)->usize{
    //find each corresponding index...
    let mut idx = loc;
    for mapping in [&data.humidity_to_location, &data.temperature_to_humidity, &data.light_to_temperature,
         &data.water_to_light, &data.fertilizer_to_water, &data.soil_to_fertilizer, &data.seed_to_soil]{
        
        //println!("Looking for {} in {:?}", idx, mapping[1]);
        let mut range_idx = 0;
        let mut offset = 0;
        let mut found = false;
        for k in 0..mapping[0].len(){
            let range = mapping[0][k];
            if idx >= range.0 && idx < range.1{
                range_idx = k;
                offset = idx - range.0;
                found = true;
                break;
            }
        }
        if found{
            idx = mapping[1][range_idx].0 + offset
        }
        //println!("Continuing with {}", idx);
    }
    idx
}


fn part2(data:DataFrame){
    //find the lowest location number
    //this works, but is too inefficient for the true dataset
    //see part2_improved for a better implementation, which works from bottom-up instead
    let mut lowest_location:usize = 9999999999;
    //now need to reformat seeds to take ranges into account
    let mut seeds:Vec<usize> = Vec::new();
    for i in (0..data.seeds.len()).step_by(2){
        seeds.append(&mut (data.seeds[i]..data.seeds[i]+data.seeds[i+1]).collect::<Vec<usize>>().clone() );
    }
    let mut best_index = 0;
    //find the range with the lowest values?
    for i in 0..seeds.len(){
        let seed = seeds[i];
        let loc = get_location(seed, &data);
        if loc < lowest_location{
            lowest_location = loc;
            best_index = i;
        }
    }
    println!("Part 2");
    println!("The smallest loc is {}", lowest_location);
    
    
}

fn part2_improved(data:DataFrame){
    //find the lowest location number
    //the first attempt started from seeds and found locations, it will be faster to start from the locations
    //edit: much faster
    let mut final_idx = 0;
    for loc_idx in 0..171183359{
        let seed = get_seed(loc_idx, &data);
        //check if seed is a valid seed
        let mut do_break = false;
        for i in (0..data.seeds.len()).step_by(2){
            if seed >= data.seeds[i] && seed <data.seeds[i]+data.seeds[i+1]{
                println!("Seed {} at loc {}", seed, loc_idx);
                final_idx = loc_idx;
                do_break = true;
                break;
            }
        }
        if do_break{
            break;
        }
    }
    println!("Part 2");
    println!("The smallest loc is {}", final_idx );
}

#[derive(Debug,Clone)]
struct DataFrame{
    seeds: Vec<usize>,
    seed_to_soil: [Vec<(usize, usize)>;2],
    soil_to_fertilizer: [Vec<(usize, usize)>;2],
    fertilizer_to_water: [Vec<(usize, usize)>;2],
    water_to_light: [Vec<(usize, usize)>;2],
    light_to_temperature: [Vec<(usize, usize)>;2],
    temperature_to_humidity: [Vec<(usize, usize)>;2],
    humidity_to_location: [Vec<(usize, usize)>;2],
}


fn process_input(filename:&str)->DataFrame{
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    //save each line as a tuple of winning numbers and my numbers    
    let mut datatype = "seeds";

    let mut seed_to_soil:[Vec<(usize, usize)>;2] = [Vec::new(), Vec::new()];
    let mut soil_to_fertilizer:[Vec<(usize, usize)>;2] = [Vec::new(), Vec::new()];
    let mut fertilizer_to_water:[Vec<(usize, usize)>;2] = [Vec::new(), Vec::new()];
    let mut water_to_light:[Vec<(usize, usize)>;2] = [Vec::new(), Vec::new()];
    let mut light_to_temperature:[Vec<(usize, usize)>;2] = [Vec::new(), Vec::new()];
    let mut temperature_to_humidity:[Vec<(usize, usize)>;2] = [Vec::new(), Vec::new()];
    let mut humidity_to_location:[Vec<(usize, usize)>;2] = [Vec::new(), Vec::new()];

    let mut seeds:Vec<usize> = Vec::new();
    
    for i in 0..lines.len(){
        let l = lines[i];
        println!("{}", l);
        if l.trim().len() < 1{
            datatype = "none"
        }

        if datatype != "none" && datatype != "seeds"{
            let values:Vec<usize> = l.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<usize>().unwrap()).collect();
            let source_vals:(usize, usize) = (values[1],values[1]+values[2]);
            let dest_vals:(usize, usize) = (values[0],values[0]+values[2]);
            if datatype == "seed-to-soil"{
                seed_to_soil[0].push(dest_vals.clone());
                seed_to_soil[1].push(source_vals.clone());
            }else if datatype == "soil-to-fertilizer"{
                soil_to_fertilizer[0].push(dest_vals.clone());
                soil_to_fertilizer[1].push(source_vals.clone());
            }else if datatype == "fertilizer-to-water"{
                fertilizer_to_water[0].push(dest_vals.clone());
                fertilizer_to_water[1].push(source_vals.clone());
            }else if datatype == "water-to-light"{
                water_to_light[0].push(dest_vals.clone());
                water_to_light[1].push(source_vals.clone());
            }else if datatype == "light-to-temperature"{
                light_to_temperature[0].push(dest_vals.clone());
                light_to_temperature[1].push(source_vals.clone());
            }else if datatype == "temperature-to-humidity"{
                temperature_to_humidity[0].push(dest_vals.clone()); 
                temperature_to_humidity[1].push(source_vals.clone());
            }else if datatype == "humidity-to-location"{
                humidity_to_location[0].push(dest_vals.clone());
                humidity_to_location[1].push(source_vals.clone());
            }
        }else if datatype == "seeds" {
            let values: Vec<usize> = l.trim().split(":").collect::<Vec<&str>>()[1]
                .split_whitespace().collect::<Vec<&str>>()
                .iter().map(|x| x.parse::<usize>().unwrap()).collect();
            seeds.append(&mut values.clone());
            datatype = "none"
        }



        if l.contains("seed-to-soil") {
            datatype = "seed-to-soil"
        }else if l.contains("soil-to-fertilizer") {
            datatype = "soil-to-fertilizer"
        }else if l.contains("fertilizer-to-water") {
            datatype = "fertilizer-to-water"
        }else if l.contains("water-to-light") {
            datatype = "water-to-light"
        }else if l.contains("light-to-temperature") {
            datatype = "light-to-temperature"
        }else if l.contains("temperature-to-humidity") {
            datatype = "temperature-to-humidity"
        }else if l.contains("humidity-to-location") {
            datatype = "humidity-to-location"
        }
    }

    let my_data = DataFrame{
        seeds: seeds,
        seed_to_soil: seed_to_soil,
        soil_to_fertilizer: soil_to_fertilizer,
        fertilizer_to_water: fertilizer_to_water,
        water_to_light: water_to_light,
        light_to_temperature: light_to_temperature,
        temperature_to_humidity: temperature_to_humidity,
        humidity_to_location: humidity_to_location,
    };

    my_data
}