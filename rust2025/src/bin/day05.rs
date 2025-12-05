use rust2025::read_lines;

fn main() {
    println!("🎄 Advent of Code 2025 - Day 5 🎄");

    // Read the file content
    let content_test = read_lines("inputs/day05_test.txt");
    let content_full = read_lines("inputs/day05.txt");

    // --- Part 1 Logic ---
    let got = solve_part1(&content_test);
    let want = 3;
    assert!(got == want, "Got {got:?}, want {want:?}");
    let part1_result = solve_part1(&content_full);
    println!("Part 1 Result: {part1_result}");

    // --- Part 2 Logic ---
    let got = solve_part2(&content_test) as usize;
    let want: usize = 14;
    assert!(got == want, "Got {got:?}, want {want:?}");
    let part2_result = solve_part2(&content_full);
    println!("Part 2 Result: {part2_result}");
}

#[derive(Debug, Clone, Copy)]
struct IDRange {
    start: usize,
    end: usize,
}

fn parse_inputs(input: &Vec<String>) -> (Vec<IDRange>, Vec<usize>) {
    let mut ranges: Vec<IDRange> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();
    let mut in_ranges = true;
    for line in input {
        if line.is_empty() {
            in_ranges = false;
            continue;
        }
        if in_ranges {
            let parts: Vec<&str> = line.split("-").collect();
            let idr: IDRange = IDRange {
                start: parts[0].parse::<usize>().unwrap_or_default(),
                end: parts[1].parse::<usize>().unwrap_or_default(),
            };
            ranges.push(idr);
        } else {
            ids.push(line.parse::<usize>().unwrap_or_default());
        }
    }
    (ranges, ids)
}

fn solve_part1(input: &Vec<String>) -> u32 {
    let mut sum = 0;
    let (ranges, ids) = parse_inputs(input);
    for id in ids {
        let mut count = 0;
        for idr in &ranges {
            if id >= idr.start && id <= idr.end {
                count += 1;
            }
        }
        if count > 0 {
            sum += 1;
        }
    }

    sum
}

fn solve_part2(input: &Vec<String>) -> usize {
    let (mut ranges, _) = parse_inputs(input);
    //brute force
    //let mut set: HashSet<usize> = HashSet::new();
    // for r in &ranges {
    //     for val in r.start..=r.end {
    //         _ = set.insert(val);
    //     }
    // }

    // 1. Sort by start of the range
    ranges.sort_by_key(|r| r.start);

    // 2. Merge overlapping ranges
    let mut merged = vec![ranges[0]];
    for r in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();

        if r.start <= last.end + 1 {
            // overlaping or touching, so we merge
            last.end = last.end.max(r.end);
        } else {
            merged.push(r);
        }
    }

    // 3. Sum lengths (inclusive ranges)
    merged.iter().map(|r| r.end - r.start + 1).sum()
}
