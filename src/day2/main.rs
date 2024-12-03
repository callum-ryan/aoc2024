use std::fs;

fn main() {
    let input = fs::read_to_string("src/input/day2.txt/").unwrap();
    let lines: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|x| {
            x.split(' ')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();

    let res1 = part1(&lines);
    println!("Day 2, Part 1: {res1}");
    let res2 = part2(&lines);
    println!("Day 2, Part 2: {res2}");
}

fn part1(lines: &Vec<Vec<i32>>) -> i32 {
    let mut n_valid = 0;
    for line in lines {
        if is_valid_sequence(line) {
            n_valid += 1;
        }
    }
    n_valid
}

fn part2(lines: &Vec<Vec<i32>>) -> i32 {
    let mut n_valid = 0;
    for line in lines {
        if is_valid_sequence(line) {
            n_valid += 1;
        } else {
            for i in 0..line.len() {
                let mut copy = line.clone();
                copy.remove(i);
                if is_valid_sequence(&copy) {
                    n_valid += 1;
                    break;
                }
            }
        }
    }
    n_valid
}

fn is_valid_sequence(seq: &[i32]) -> bool {
    let is_increasing = seq.windows(2).all(|x| x[1] > x[0]);
    let is_decreasing = seq.windows(2).all(|x| x[1] < x[0]);
    if is_decreasing || is_increasing {
        let jump_size = seq
            .windows(2)
            .all(|x| ((x[1] - x[0]).abs() >= 1) && ((x[1] - x[0]).abs() <= 3));

        if jump_size {
            return true;
        }
    }
    false
}
