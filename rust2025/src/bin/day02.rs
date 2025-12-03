use std::{cmp::min, fmt::format};

use rust2025::read_lines;

fn main() {
    println!("🎄 Advent of Code 2025 - Day 2 🎄");

    // Read the file content
    let content_test = read_lines("inputs/day02_test.txt");
    let content_full = read_lines("inputs/day02.txt");

    // --- Part 1 Logic ---
    let got = solve_part1(&content_test);
    let want = 1227775554;
    assert!(got == want, "Got {:?}, want {:?}", got, want);
    let part1_result = solve_part1(&content_full);
    println!("Part 1 Result: {}", part1_result);

    // --- Part 2 Logic ---
    let _ = check_substrings("565656");
    let got = solve_part2(&content_test);
    let want = 4174379265;
    assert!(got == want, "Got {:?}, want {:?}", got, want);
    let part2_result = solve_part2(&content_full);
    println!("Part 2 Result: {}", part2_result);
}

#[derive(Debug)]
struct IDRange {
    start: u64,
    end: u64,
}

fn parse_line(input: &str) -> Vec<IDRange> {
    let ranges: Vec<IDRange> = input
        .split(",")
        .map(|r| {
            let parts: Vec<&str> = r.split("-").collect();
            let idr: IDRange = IDRange {
                start: parts[0].parse::<u64>().unwrap_or_default(),
                end: parts[1].parse::<u64>().unwrap_or_default(),
            };
            idr
        })
        .collect();
    ranges
}

fn solve_part1(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    let ranges = parse_line(&input[0]);
    //for each number:
    //1. make into a string
    //2. split in the middle
    //3. check if first half equals second half
    //5. if number is odd length, continue
    for range in ranges {
        let start = range.start;
        let end = range.end;
        for i in start..end + 1 {
            let s = format!("{}", i);
            if s.len() % 2 != 0 {
                continue;
            }
            if s[..s.len() / 2] == s[s.len() / 2..] {
                sum += i
            }
        }
    }

    sum
}

fn solve_part2(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    let ranges = parse_line(&input[0]);
    //for each number:
    //1. make into a string
    //2. check if a sequence is repeated at least twice
    for range in ranges {
        let start = range.start;
        let end = range.end;
        for i in start..end + 1 {
            let s = format!("{}", i);
            if check_substrings(&s) {
                sum += i
            }
        }
    }

    sum
}

fn check_substrings(s: &str) -> bool {
    let start_idx = 0;
    for length in 1..=s.len() / 2 {
        let mut is_valid = false;
        // the first section of length length
        let substr1 = &s[start_idx..start_idx + length];
        // the next section of length length
        for substr_idx in (start_idx + length..s.len()).step_by(length) {
            let substr2 = &s[substr_idx..min(substr_idx + length, s.len())];
            if substr1 == substr2 {
                is_valid = true;
            } else {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            // println!("Flagged {}", s);
            return true;
        }
    }
    false
}
