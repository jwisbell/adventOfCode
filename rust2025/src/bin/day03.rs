use rust2025::read_lines;

fn main() {
    println!("🎄 Advent of Code 2025 - Day 3 🎄");

    // Read the file content
    let content_test = read_lines("inputs/day03_test.txt");
    let content_full = read_lines("inputs/day03.txt");

    // --- Part 1 Logic ---
    let got = solve_part1(&content_test);
    let want = 357;
    assert!(got == want, "Got {got:?}, want {want:?}");
    let part1_result = solve_part1(&content_full);
    println!("Part 1 Result: {part1_result}");

    // --- Part 2 Logic ---
    let got = solve_part2(&content_test) as u64;
    let want: u64 = 3121910778619;
    assert!(got == want, "Got {got:?}, want {want:?}");
    let part2_result = solve_part2(&content_full);
    println!("Part 2 Result: {part2_result}");
}

fn parse_battery_bank(input: &str) -> Vec<u32> {
    let batteries: Vec<u32> = input
        .chars()
        .map(|x| String::from(x).parse::<u32>().unwrap_or_default())
        .collect();
    batteries
}

fn solve_part1(input: &Vec<String>) -> u32 {
    let mut sum = 0;
    //for each number:
    //1. make into a string
    //2. split in the middle
    //3. check if first half equals second half
    //5. if number is odd length, continue
    for row in input {
        let batteries = parse_battery_bank(&row);
        if batteries.is_empty() {
            continue;
        }
        //want to traverse the list once, keeping track of the highest values
        let mut highest_val = 0;
        let mut joltage = 0;
        for idx in 0..batteries.len() {
            if batteries[idx] > highest_val && idx != batteries.len() - 1 {
                highest_val = batteries[idx];
            } else {
                let temp_joltage = format!("{}{}", highest_val, batteries[idx])
                    .parse::<u32>()
                    .unwrap_or_default();
                if temp_joltage > joltage {
                    joltage = temp_joltage;
                }
            }
        }
        sum += joltage;
    }
    sum
}

fn solve_part2(input: &Vec<String>) -> u64 {
    let mut sum = 0;

    for row in input {
        let batteries = parse_battery_bank(&row);
        if batteries.is_empty() {
            continue;
        }
        //want to traverse the list once, keeping track of the highest values
        sum += sliding_box(&batteries);
    }
    sum
}

fn sliding_box(batteries: &Vec<u32>) -> u64 {
    let mut digits: Vec<u32> = Vec::new();

    let mut start_idx = 0;

    for num_length in (0..12).rev() {
        let mut highest_idx = 0;
        let mut highest_val = 0;
        for idx in start_idx..batteries.len() {
            if batteries[idx] > highest_val && idx < batteries.len() - num_length {
                highest_idx = idx;
                highest_val = batteries[idx];
            }
        }
        digits.push(highest_val);
        start_idx = highest_idx + 1;
    }
    let s: Vec<String> = digits.iter().map(|x| format!("{x}")).collect();
    let combined = s.join("");
    combined.parse::<u64>().unwrap_or_default()
}
