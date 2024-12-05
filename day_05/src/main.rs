use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn reader(input_file: String, dict: &mut HashMap<i32, Vec<i32>>, updates: &mut Vec<Vec<i32>>) {
    let file = File::open(&input_file).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let split_index = lines
        .iter()
        .position(|line| line.is_empty())
        .unwrap_or(lines.len());

    for line in &lines[..split_index] {
        let mut split = line.split("|");
        let key = split.next().unwrap().parse::<i32>().unwrap();
        let value = split.next().unwrap().parse::<i32>().unwrap();
        dict.entry(key).or_insert(Vec::new()).push(value);
    }

    for line in &lines[split_index + 1..] {
        updates.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
    }
}

fn main() {
    let mut dict: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    reader("example.txt".to_string(), &mut dict, &mut updates);
    println!("{:?}", dict);
    println!("{:?}", updates);
}