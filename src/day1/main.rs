use std::collections::HashMap;
use std::fs;
use std::ops::Index;

fn main() {
    let input = fs::read_to_string("src/day1/input.txt").unwrap();
    let lines: Vec<String> = input.trim().lines().map(|x| x.to_string()).collect();

    let res1 = part1(lines.clone());
    let res2: i32 = part2(lines.clone());

    println!("Day 1, Part 1: {res1}");
    println!("Day 1, Part 2: {res2}");
}

fn part1(lines: Vec<String>) -> i32 {
    let mut col1: Vec<i32> = Vec::with_capacity(lines.len());
    let mut col2: Vec<i32> = Vec::with_capacity(lines.len());

    for line in lines {
        let (c1, c2) = line.split_once("   ").unwrap();
        col1.push(c1.parse::<i32>().unwrap());
        col2.push(c2.parse::<i32>().unwrap());
    }

    col1.sort();
    col2.sort();

    let mut diff_sum = 0;

    for i in 0..col1.len() {
        let diff: i32 = (col1.index(i) - col2.index(i)).abs();

        diff_sum += diff;
    }

    diff_sum as i32
}

fn part2(lines: Vec<String>) -> i32 {
    let mut col1: Vec<i32> = Vec::with_capacity(lines.len());
    let mut col2: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let (c1, c2) = line.split_once("   ").unwrap();
        let c1i = c1.parse::<i32>().unwrap();
        let c2i: i32 = c2.parse::<i32>().unwrap();

        col1.push(c1i);

        if let Some(cnt) = col2.get(&c2i) {
            col2.insert(c2i.clone(), cnt.clone() + 1);
        } else {
            col2.insert(c2i.clone(), 1);
        }
    }

    let mut sum = 0;

    for value in col1 {
        if let Some(count) = col2.get(&value) {
            sum += count * value;
        } else {
            sum += 0;
        }
    }

    sum
}
