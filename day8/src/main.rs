use std::fs;
use std::collections::HashMap;
use std::cmp;

#[derive(Debug, Clone)]
struct Input {
    frequencies: HashMap<char, Vec<(i32, i32)>>,
    antinodes: Vec<Vec<bool>>
}

fn main() {
    let mut input_1 = read_input();
    let mut input_2 = input_1.clone();

    part1(&mut input_1);
    part2(&mut input_2);
}

fn read_input() -> Input {
    let mut frequencies: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut rows = 0;
    let mut cols = 0;

    fs::read_to_string("input.txt").unwrap().lines()
        .enumerate()
        .for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                cols = cmp::max(cols, j + 1);

                if c == '.' {
                    return;
                }

                let vec = frequencies.entry(c).or_insert_with(|| Vec::new());
                vec.push((i as i32, j as i32));
            });

            rows = cmp::max(rows, i + 1);
        });

    let antinodes = vec![vec![false; cols]; rows];

    Input { frequencies, antinodes }
}

fn in_bounds(coords: (i32, i32), antinodes: &Vec<Vec<bool>>) -> bool {
    coords.0 >= 0 && coords.0 < antinodes.len() as i32 && coords.1 >= 0 && coords.1 < antinodes[0].len() as i32
}

fn compute_antinodes_1(antenna_1: (i32, i32), antenna_2: (i32, i32), antinodes: &mut Vec<Vec<bool>>) {
    let dr = antenna_1.0 - antenna_2.0;
    let dc = antenna_1.1 - antenna_2.1;

    let antinode_1 = (antenna_1.0 + dr, antenna_1.1 + dc);
    let antinode_2 = (antenna_2.0 - dr, antenna_2.1 - dc);

    if in_bounds(antinode_1, antinodes) {
        antinodes[antinode_1.0 as usize][antinode_1.1 as usize] = true;
    }

    if in_bounds(antinode_2, antinodes) {
        antinodes[antinode_2.0 as usize][antinode_2.1 as usize] = true;
    }
}

fn compute_antinodes_2(antenna_1: (i32, i32), antenna_2: (i32, i32), antinodes: &mut Vec<Vec<bool>>) {
    let dr = antenna_1.0 - antenna_2.0;
    let dc = antenna_1.1 - antenna_2.1;

    let mut antinode_1 = (antenna_1.0, antenna_1.1);

    while in_bounds(antinode_1, antinodes) {
        antinodes[antinode_1.0 as usize][antinode_1.1 as usize] = true;

        antinode_1.0 += dr;
        antinode_1.1 += dc;
    }

    let mut antinode_2 = (antenna_2.0, antenna_2.1);

    while in_bounds(antinode_2, antinodes) {
        antinodes[antinode_2.0 as usize][antinode_2.1 as usize] = true;
        
        antinode_2.0 -= dr;
        antinode_2.1 -= dc;
    }
}

fn part1(input: &mut Input) {
    input.frequencies.iter().for_each(|(freq, antennas)| {
        (0..(antennas.len() - 1)).for_each(|i| {
            ((i + 1)..antennas.len()).for_each(|j| {
                compute_antinodes_1(antennas[i], antennas[j], &mut input.antinodes);
            })
        })
    });

    let ans: usize = input.antinodes.iter().map(|vec| {
        vec.iter().filter(|e| **e).count()
    }).sum();

    println!("Part 1: {ans}");
}

fn part2(input: &mut Input) {
    input.frequencies.iter().for_each(|(freq, antennas)| {
        (0..(antennas.len() - 1)).for_each(|i| {
            ((i + 1)..antennas.len()).for_each(|j| {
                compute_antinodes_2(antennas[i], antennas[j], &mut input.antinodes);
            })
        })
    });

    let ans: usize = input.antinodes.iter().map(|vec| {
        vec.iter().filter(|e| **e).count()
    }).sum();

    println!("Part 1: {ans}");
}
