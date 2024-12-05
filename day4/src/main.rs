use std::fs;
use itertools::Itertools;

struct Input {
    word_search: Vec<Vec<char>>
}

impl Input {
    fn iter(&self) -> impl Iterator<Item=(usize, usize)> {
        (0..self.word_search.len()).cartesian_product(0..self.word_search[0].len())
    }

    fn get_max_i(&self) -> usize {
        self.word_search.len()
    }

    fn get_max_j(&self) -> usize {
        self.word_search[0].len()
    }

    fn try_get(&self, i: i32, j: i32) -> char {
        if i >= 0 && i < self.get_max_i() as i32 && j >= 0 && j < self.get_max_j() as i32 {
            self.get(i as usize, j as usize)
        } else {
            '.'
        }
    }

    fn get(&self, i: usize, j: usize) -> char {
        self.word_search[i][j]
    }
}

fn main() {
    let input = read_input();

    part1(&input);
    part2(&input);
}

fn read_input() -> Input {
    let contents = fs::read_to_string("input.txt").unwrap();

    Input {
        word_search: contents.lines().map(|line| {
            line.chars().collect()
        }).collect()
    }
}

fn find_xmas_directional(input: &Input, i: usize, j: usize, di: i32, dj: i32) -> bool {
    let mut i: i32 = i as i32;
    let mut j: i32 = j as i32;

    "XMAS".chars().all(|c| {
        let is_match = input.try_get(i, j) == c;
            
        i = i + di;
        j = j + dj;

        is_match
    })
}

fn find_xmas(input: &Input, i: usize, j: usize) -> i32 {
    let mut xmas_count = 0;

    xmas_count += find_xmas_directional(input, i, j, 1, 0) as i32;
    xmas_count += find_xmas_directional(input, i, j, -1, 0) as i32;
    xmas_count += find_xmas_directional(input, i, j, 0, 1) as i32;
    xmas_count += find_xmas_directional(input, i, j, 0, -1) as i32;
    xmas_count += find_xmas_directional(input, i, j, 1, 1) as i32;
    xmas_count += find_xmas_directional(input, i, j, -1, 1) as i32;
    xmas_count += find_xmas_directional(input, i, j, 1, -1) as i32;
    xmas_count += find_xmas_directional(input, i, j, -1, -1) as i32;

    return xmas_count;
}

fn find_x_mas_directional(input: &Input, i: i32, j: i32, sign: i32) -> bool {
    let has_m = input.try_get(i - sign, j - 1) == 'M' || input.try_get(i + sign, j + 1) == 'M';
    let has_s = input.try_get(i - sign, j - 1) == 'S' || input.try_get(i + sign, j + 1) == 'S';
    let has_a = input.try_get(i, j) == 'A';

    has_m && has_a && has_s
}

fn find_x_mas(input: &Input, i: usize, j: usize) -> bool {
    let i = i as i32;
    let j = j as i32;

    find_x_mas_directional(input, i, j, 1) && find_x_mas_directional(input, i, j, -1)
}

fn part1(input: &Input) {
    let num_xmas: i32 = input.iter().map(|(i, j)| {
        find_xmas(input, i, j)
    }).sum();

    println!("Part 1: {num_xmas}");
}

fn part2(input: &Input) {
    let num_x_mas: i32 = input.iter().map(|(i, j)| {
        find_x_mas(input, i, j) as i32
    }).sum();

    println!("Part 2: {num_x_mas}");
}