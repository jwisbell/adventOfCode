use std::cmp::{max, min};

use rust2025::read_lines;

fn main() {
    println!("🎄 Advent of Code 2025 - Day 4 🎄");

    // Read the file content
    let content_test = read_lines("inputs/day04_test.txt");
    let content_full = read_lines("inputs/day04.txt");

    // --- Part 1 Logic ---
    let got = solve_part1(&content_test);
    let want = 13;
    assert!(got == want, "Got {got:?}, want {want:?}");
    let part1_result = solve_part1(&content_full);
    println!("Part 1 Result: {part1_result}");

    // --- Part 2 Logic ---
    let got = solve_part2(&content_test) as u64;
    let want: u64 = 43;
    assert!(got == want, "Got {got:?}, want {want:?}");
    let part2_result = solve_part2(&content_full);
    println!("Part 2 Result: {part2_result}");
}

fn check_adjacent(grid: &Vec<String>, x: isize, y: isize) -> bool {
    let max_neighbors = 3;
    let mut count = 0;
    let symbol: char = '@';

    for xc in max(0, x - 1)..min(grid[0].len() as isize, x + 2) {
        for yc in max(0, y - 1)..min(grid.len() as isize, y + 2) {
            if xc == x && yc == y {
                continue;
            }

            if grid[yc as usize].chars().nth(xc as usize).unwrap() == symbol {
                count += 1;
                if count > max_neighbors {
                    return false;
                }
            }
        }
    }
    true
}

fn solve_part1(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    //pass over the grid and find all @ symbols that have
    //fewer than four other @ in the 8 adjacent positions
    let symbol: char = '@';

    let mut locations: Vec<(usize, usize)> = Vec::new();
    let mut loc_grid = input.clone();

    for (y, row) in input.iter().enumerate() {
        for (x, val) in row.chars().enumerate() {
            if val == symbol {
                //check the adjacent
                let valid = check_adjacent(input, x as isize, y as isize);
                if valid {
                    sum += 1;
                    locations.push((x, y));
                    loc_grid[y].replace_range(x..x + 1, "x");
                }
            }
        }
    }

    //print the grid
    for row in input {
        println!("{row:?}");
    }
    //print found locations
    for row in loc_grid {
        println!("{row:?}");
    }

    sum
}

fn solve_part2(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    //pass over the grid and find all @ symbols that have
    //fewer than four other @ in the 8 adjacent positions
    let symbol: char = '@';

    let mut temp_grid = input.clone();
    loop {
        let mut locations: Vec<(usize, usize)> = Vec::new();

        for (y, row) in temp_grid.iter().enumerate() {
            for (x, val) in row.chars().enumerate() {
                if val == symbol {
                    //check the adjacent
                    let valid = check_adjacent(&temp_grid, x as isize, y as isize);
                    if valid {
                        sum += 1;
                        locations.push((x, y));
                    }
                }
            }
        }
        if locations.is_empty() {
            break;
        } else {
            for (x, y) in locations {
                temp_grid[y].replace_range(x..x + 1, ".");
            }
            //print the grid
            for row in temp_grid.clone() {
                println!("{row:?}");
            }
        }
    }

    //print the grid
    for row in input {
        println!("{row:?}");
    }

    sum
}
