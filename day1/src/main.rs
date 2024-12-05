use std::fs;
use std::collections::HashMap;
use std::vec::IntoIter;
use tuple_conv::RepeatedTuple;
use itertools::Itertools;
use itertools::multizip;

fn main() {
    part1();
    part2();
}

fn part1() {
    let ans: i32 = multizip(fs::read_to_string("input.txt").unwrap()
    .split("\n")
    .map(|line| line.split("   ")
        .map(|num| 
            num.parse::<i32>().unwrap()
        ).collect_tuple().unwrap()
    ).unzip()
    .to_vec()
    .into_iter()
    .map(|mut vec: Vec<i32>| {
        vec.sort(); 
        vec.into_iter()
    }).collect_tuple::<(IntoIter<i32>, IntoIter<i32>)>()
    .unwrap())
    .map(|(l, r)| 
        (l - r).abs()
    ).sum();

    println!("Part 1: {ans}");
}

fn part2() {
    let (left, right): (Vec<i32>, Vec<i32>) = fs::read_to_string("input.txt").unwrap()
        .split("\n")
        .map(|line| line.split("   ")
        .map(|num| 
            num.parse::<i32>().unwrap()
        ).collect_tuple().unwrap()
    ).unzip();

    let mut right_freq = HashMap::new();

    right.into_iter().for_each(|num| {
        let freq: &mut i32 = right_freq.entry(num).or_insert(0);
        *freq += 1;
    });

    let ans = left.into_iter().fold(0, |acc, e| acc + e * *right_freq.entry(e).or_default());
    println!("Part 2: {ans}");
}