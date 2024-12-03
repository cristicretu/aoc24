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

fn main() {
    let mut corr_str = String::new();
    reader("1.txt".to_string(), &mut corr_str);
    println!("{}", corr_str);

    let re = Regex::new(r"(mul\(([\d,]+)\)|don\'t\(\)|do\(\))").unwrap();
    let matches = re.find_iter(&corr_str);
    let mut ans1 = 0;
    let mut ok = true;
    for mat in matches {
        println!("{}", mat.as_str());
        let ln = mat.as_str();
        if ln.contains("mul") && ok {
            let ln = &ln[4..ln.len() - 1];
            let numbers: Vec<i32> = ln.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            ans1 += numbers[0] * numbers[1];
        } else if ln.contains("don't") {
            ok = false;
        } else if ln.contains("do") {
            ok = true;
        }
    }
    println!("{}", ans1);
}
