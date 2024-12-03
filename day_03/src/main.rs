use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn reader(input_file: String, corr_str: &mut String) {
    let file = File::open(input_file).unwrap();
    // read only one line
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let ln = line.unwrap();
        *corr_str += &ln.trim().to_string();
    }
}

fn solve1(corr_str: &str) -> i32 {
    let re = Regex::new(r"mul\(([\d,]+)\)").unwrap();
    let matches = re.find_iter(&corr_str);
    let mut sum = 0;
    for mat in matches {
        let ln = mat.as_str();
        let ln = &ln[4..ln.len() - 1];
        let numbers: Vec<i32> = ln.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        sum += numbers[0] * numbers[1];
    }
    sum
}

fn solve2(corr_str: &str) -> i32 {
    let re: Regex = Regex::new(r"(mul\(([\d,]+)\)|don\'t\(\)|do\(\))").unwrap();
    let matches = re.find_iter(&corr_str);
    let mut ok = true;

    let mut sum = 0;

    for mat in matches {
        let ln = mat.as_str();
        if ln.contains("mul") && ok {
            let ln = &ln[4..ln.len() - 1];
            let numbers: Vec<i32> = ln.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            sum += numbers[0] * numbers[1];
        } else if ln.contains("don't") {
            ok = false;
        } else if ln.contains("do") {
            ok = true;
        }
    }
    sum
}

fn main() {
    let mut corr_str = String::new();
    reader("1.txt".to_string(), &mut corr_str);

    let ans1 = solve1(&corr_str);
    let ans2 = solve2(&corr_str);
    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}
