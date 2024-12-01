use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn reader(input_file: String, pile1: &mut Vec<i32>, pile2: &mut Vec<i32>) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let ln = line.unwrap();
        let numbers: Vec<i32> = ln
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let number1 = numbers[0];
        let number2 = numbers[1];

        pile1.push(number1);
        pile2.push(number2);
    }
}

fn solve1(pile1: &mut Vec<i32>, pile2: &mut Vec<i32>) -> i32 {
    pile1.sort();
    pile2.sort();

    let mut sum = 0;
    for (a, b) in pile1.iter().zip(pile2.iter()) {
        sum = sum + (a - b).abs();
    }

    sum
}

fn solve2(pile1: &mut Vec<i32>, pile2: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut hm: HashMap<i32, i32> = HashMap::new();

    for a in pile2.iter() {
        if hm.contains_key(a) {
            *hm.get_mut(a).unwrap() += 1;
        } else {
            hm.insert(*a, 1);
        }
    }

    for a in pile1.iter() {
        sum = sum + a * hm.get(a).unwrap_or(&0);
    }

    sum
}

fn main() {
    let mut pile1: Vec<i32> = Vec::new();
    let mut pile2: Vec<i32> = Vec::new();

    reader("1.txt".to_string(), &mut pile1, &mut pile2);

    let ans1 = solve1(&mut pile1, &mut pile2);
    let ans2 = solve2(&mut pile1, &mut pile2);

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}
