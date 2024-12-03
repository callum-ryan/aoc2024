use std::fs;

use regex::Regex;

fn main() {
    let input: String = fs::read_to_string("src/input/day3.txt").unwrap();
    let lines: String = input.trim().to_string();

    let res1: i32 = part1(&lines);
    let res2: i32 = part2(&lines);

    println!("Day 1, Part 1: {res1}");
    println!("Day 1, Part 2: {res2}");
}

fn part1(lines: &str) -> i32 {
    let pat = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    pat.captures_iter(lines)
        .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        .sum()
}

fn part2(lines: &str) -> i32 {
    let mut sum = 0;
    let (starting, remainder) = lines.split_once("don't()").unwrap();
    sum += part1(starting);
    let (_, rest) = remainder.split_once("do()").unwrap_or_default();
    if !rest.is_empty() {
        sum += part2(rest);
    }
    sum
}
