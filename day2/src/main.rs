use std::fs;
use std::iter;

struct Input {
    reports: Vec<Vec<i32>>
}

fn main() {
   let input = read_input();
   
   part1(&input);
   part2(&input);
}

fn read_input() -> Input {
    let contents = fs::read_to_string("input.txt").unwrap();

    let reports = contents.lines()
        .map(|line| {
            line.split(" ").map(|level| {
                level.parse::<i32>().unwrap()
            }).collect()
        }).collect();

    Input { reports }
}

fn is_safe(report: &Vec<i32>, ignore_index: usize) -> bool {
    let mut prev_val = -1;
    let mut trend: i32 = 0;
    let mut safe = true;

    for (i, &e) in report[0..].iter().enumerate() {
        if i == ignore_index {
            continue;
        }

        if prev_val == -1 {
            prev_val = e;
            continue;
        }

        let dist = (e - prev_val).abs();

        safe &= (dist >= 1 && dist <= 3);

        if (trend == 0) {
            if (e > prev_val) {
                trend = 1;
            }

            if (e < prev_val) {
                trend = -1;
            }
        } else {
            if (e > prev_val) {
                safe &= (trend == 1);
            }

            if (e < prev_val) {
                safe &= (trend == -1);
            }
        }

        prev_val = e;
    }

    safe
}

fn part1(input: &Input) {
    let num_safe = input.reports.iter()
        .filter(|report| is_safe(report, usize::MAX))
        .count();

    println!("Part 1: {num_safe}");
}

fn part2(input: &Input) {
    let num_safe = input.reports.iter()
        .filter(|report| {
            (0..report.len()).chain(iter::once(usize::MAX)).any(|i| {
                is_safe(report, i)
            })
        })
        .count();

    println!("Part 2: {num_safe}");
}
