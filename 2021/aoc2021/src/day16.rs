use std::fs;
//use std::collections::HashMap;
use regex::{Captures, Regex, RegexSet};



pub fn main(){
    //let filename = "./inputs/day16_test.txt";
    let filename = "./inputs/day16.txt";
    
    let hex = process_file(filename);
    println!("{}", hex);
    part_one(&hex);
    
}

fn part_one(hex: &str){

    //use regex to replace the hex values with binary
    let re = Regex::new("(0|1|2|3|4|5|6|7|8|9|A|B|C|D|E|F)").unwrap();
    let string = hex.clone();
    let result = re.replace_all(string, |cap: &Captures| {
        match &cap[0] {
            "0" => "0000",
            "1" => "0001",
            "2" => "0010",
            "3" => "0011",
            "4" => "0100",
            "5" => "0101",
            "6" => "0110",
            "7" => "0111",
            "8" => "1000",
            "9" => "1001",
            "A" => "1010",
            "B" => "1011",
            "C" => "1100",
            "D" => "1101",
            "E" => "1110",
            "F" => "1111",
            _ => panic!("We should never get here"),
        }.to_string()
    });
    println!("{}", result);
    




    let version = result.get(0..3).unwrap();
    let id = result.get(3..6).unwrap();
    let v_int = usize::from_str_radix(&version, 2).unwrap();
    let id_int = usize::from_str_radix(&id, 2).unwrap();
    if id_int == 4{
        println!("{} {}", v_int, id_int);
        let mut components: Vec<String> = Vec::new();
        let mut i = 6;
        while i < result.len()-4{
            let temp = result.get(i+1..i+5).unwrap();
            components.push(String::from(temp));
            i+=5
        }
        let value = String::from_iter( components );
        println!("{:?}", usize::from_str_radix(&value, 2).unwrap() );
    }
    

    //then parse the binary?
    println!("{:?}",parse_packet(&result, result.len()));


}

fn parse_packet(packet: &str, size:usize) -> (usize,usize) {
    println!("{}",packet);
    let version = packet.get(0..3).unwrap();
    let id = packet.get(3..6).unwrap();
    let v_int = usize::from_str_radix(&version, 2).unwrap();
    let id_int = usize::from_str_radix(&id, 2).unwrap();
    if id_int == 4{
        println!("{} {}", v_int, id_int);
        let mut components: Vec<String> = Vec::new();
        let mut i = 6;
        if packet.get(i..i+1).unwrap() == "0"{
            let temp = packet.get(i+1..i+5).unwrap();
            components.push(String::from(temp));
            let value = String::from_iter( components );
            println!("literal {}",value);
            //return (usize::from_str_radix(&value, 2).unwrap(), 11)  ;
            return (usize::from_str_radix(&version, 2).unwrap(), 11)  ;
        }
        while packet.get(i..i+1).unwrap() == "1" {
            let temp = packet.get(i+1..i+5).unwrap();
            components.push(String::from(temp));
            i+=5
        }
        let value = String::from_iter( components );
        println!("literal {}",value);
        //return (usize::from_str_radix(&value, 2).unwrap(), i+5)  ; 
        return (usize::from_str_radix(&version, 2).unwrap(), i+5)  ; 
    }else{
        let len_id = packet.get(6..7).unwrap();
        if len_id == "0"{
            //println!("15 version");
            //next 15 digits are len in bits of all subpackets
            let subpacket_len = usize::from_str_radix(&packet.get(7..22).unwrap(), 2).unwrap();
            println!("15 version, len {}", subpacket_len);
            let mut processed = 0;
            let mut values: Vec<usize> = Vec::new();
            values.push(v_int);
            while processed < subpacket_len{
                let (value, nbits) = parse_packet( packet.get(22+processed..22+subpacket_len).unwrap() , subpacket_len);
                values.push(value);
                processed += nbits;
            }
            println!("returning from 15");
            return (sum(&values), subpacket_len+22);
        }else{
            //next 11 bits is number of subpackets
            println!("11 version");
            let subpacket_num = usize::from_str_radix(&packet.get(7..18).unwrap(), 2).unwrap();
            println!("11 version, len {}", subpacket_num);
            let mut processed = 0;
            let mut n_processed = 0;
            let mut values: Vec<usize> = Vec::new();
            values.push(v_int);
            while n_processed < subpacket_num{
                println!("sub packet number {}", n_processed);
                let (value, nbits) = parse_packet( packet.get(18+processed..packet.len()).unwrap(), packet.len()-18+processed  );
                values.push(value);
                processed += nbits;
                n_processed += 1;
            }
            println!("returning from 11");
            return (sum(&values), processed+18);
        }

    }
    

}

fn sum(v:&Vec<usize> ) -> usize{
    let mut z = 0;
    for x in v{
        z += x;
    }
    z
}


//read the input data, 
fn process_file(filename: &str) -> String {

    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let hex: String = lines[0].trim().to_string();

    
    hex
}