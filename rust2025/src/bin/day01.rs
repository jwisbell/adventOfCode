use rust2025::read_lines;

fn main() {
    println!("🎄 Advent of Code 2025 - Day 1 🎄");

    // Read the file content
    let content_test = read_lines("inputs/day01_test.txt");
    let content_full = read_lines("inputs/day01.txt");

    // --- Part 1 Logic ---
    let got = solve_part1(&content_test);
    let want = 3;
    assert!(got == want, "Got {:?}, want {:?}", got, want);
    let part1_result = solve_part1(&content_full);
    println!("Part 1 Result: {}", part1_result);

    // --- Part 2 Logic ---
    let got = solve_part2(&content_test);
    let want = 6;
    assert!(got == want, "Got {:?}, want {:?}", got, want);
    let part2_result = solve_part2(&content_full);
    println!("Part 2 Result: {}", part2_result);
}

fn solve_part1(input: &Vec<String>) -> u32 {
    let mut loc = 50;
    let mut zeroes: u32 = 0;

    for line in input {
        if line.len() == 0 {
            continue;
        }
        let sign = line.starts_with(|c| c == 'R');
        let mut dist = line[1..].parse::<i32>().unwrap();

        if !sign {
            dist *= -1
        }

        loc += dist;
        loc = loc.rem_euclid(100);
        if loc == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn solve_part2(input: &Vec<String>) -> u32 {
    let mut loc = 50;
    let mut zeroes: u32 = 0;

    for line in input {
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next().unwrap();
        let mut dist = line[1..].parse::<i32>().unwrap();

        if dir == 'L' {
            dist = -dist;
        }

        let start = loc; // 0..99
        let end_raw = loc + dist; // signed raw end

        let crosses: i32 = if dist > 0 {
            end_raw.div_euclid(100) - start.div_euclid(100)
        } else if dist < 0 {
            // use start - 1 to exclude the starting position when start is exactly a multiple
            (start - 1).div_euclid(100) - end_raw.div_euclid(100)
        } else {
            0
        };

        zeroes += crosses.abs() as u32;

        if dist < 0 && end_raw.rem_euclid(100) == 0 {
            zeroes += 1;
        }

        // Normalize location into 0..99
        loc = end_raw.rem_euclid(100);
    }

    zeroes
}
