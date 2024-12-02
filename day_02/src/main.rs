use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

fn reader(input_file: String, mat: &mut Vec<Vec<i32>>) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let ln = line.unwrap();
        let line: Vec<i32> = ln
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        mat.push(line);
    }
}

fn solve1(mat: &mut Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for line in mat {
        let direction = (line[1] - line[0]) > 0; // increasing

        let mut add = 1;
        for i in 1..line.len() {
            if (line[i] - line[i - 1] > 0) != direction {
                add = 0;
                break;
            }
            let diff = (line[i] - line[i - 1]).abs();
            if diff > 3 || diff == 0 {
                add = 0;
                break;
            }
        }

        sum = sum + add;
    }

    sum
}

fn solve2(mat: &mut Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for line in mat {
        let direction = (line[1] - line[0]) > 0; // increasing

        let mut add = 2;
        let mut i = 1;
        while i < line.len() && add > 0 {
            let diff = (line[i] - line[i - 1]).abs();
            let mut removed = false;
            if diff > 3 || diff == 0 {
                add -= 1;
                removed = true;
                line.remove(i);
            } else if (line[i] - line[i - 1] > 0) != direction {
                add -= 1;
                removed = true;
                line.remove(i);
            }
            if !removed {
                i += 1;
            }
        }

        sum = sum + min(add, 1);
    }

    sum
}

fn main() {
    let mut mat: Vec<Vec<i32>> = Vec::new();

    reader("1.txt".to_string(), &mut mat);

    let ans1 = solve1(&mut mat);
    let ans2 = solve2(&mut mat);

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}
