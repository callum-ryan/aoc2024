use std::collections::HashMap;
use std::fs;
use std::ops::Index;

fn main() {
    let input = fs::read_to_string("src/input/day1.txt").unwrap();
    let lines: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|x| {
            x.split("   ")
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();

    let res1: i32 = part1(lines.clone());
    let res2: i32 = part2(lines.clone());

    println!("Day 1, Part 1: {res1}");
    println!("Day 1, Part 2: {res2}");
}

fn part1(lines: Vec<Vec<i32>>) -> i32 {
    let mut col1: Vec<i32> = Vec::with_capacity(lines.len());
    let mut col2: Vec<i32> = Vec::with_capacity(lines.len());

    for line in lines {
        col1.push(line[0]);
        col2.push(line[1]);
    }

    col1.sort();
    col2.sort();

    let mut diff_sum = 0;

    for i in 0..col1.len() {
        let diff: i32 = (col1.index(i) - col2.index(i)).abs();

        diff_sum += diff;
    }

    diff_sum
}

fn part2(lines: Vec<Vec<i32>>) -> i32 {
    let mut col1: Vec<i32> = Vec::with_capacity(lines.len());
    let mut col2: HashMap<i32, i32> = HashMap::with_capacity(lines.len());

    for x in lines.iter() {
        col1.push(x[0]);
        *col2.entry(x[1]).or_insert(0) += 1;
    }

    col1.iter().map(|x| col2.get(x).unwrap_or(&0) * x).sum()
}
