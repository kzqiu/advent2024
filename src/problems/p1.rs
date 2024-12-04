use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let list: Vec<Vec<i32>> = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut first: Vec<i32> = list.iter().map(|p| p[0]).collect();
    let mut second: Vec<i32> = list.iter().map(|p| p[1]).collect();

    first.sort();
    second.sort();

    let res_a: i32 = first
        .iter()
        .zip(second.iter())
        .by_ref()
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Part A: {}", res_a);

    let mut first_counts: HashMap<i32, i32> = HashMap::new();
    let mut second_counts: HashMap<i32, i32> = HashMap::new();

    for (a, b) in first.iter().zip(second.iter()).by_ref() {
        *first_counts.entry(*a).or_insert(0) += 1;
        *second_counts.entry(*b).or_insert(0) += 1;
    }

    let res_b: i32 = first_counts
        .into_iter()
        .map(|(k, _)| match second_counts.get(&k) {
            Some(v) => k * v,
            None => 0,
        })
        .sum();

    println!("Part B: {}", res_b);
}
