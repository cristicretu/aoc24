use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
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

fn process(dict: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut all_numbers = HashSet::new();
    for (&key, values) in dict.iter() {
        all_numbers.insert(key);
        all_numbers.extend(values);
    }

    // Find the "root" nodes - numbers that don't appear as values in any rule
    let mut root_nodes: Vec<i32> = all_numbers
        .iter()
        .filter(|&&num| !dict.values().any(|values| values.contains(&num)))
        .cloned()
        .collect();

    root_nodes.sort_by(|a, b| b.cmp(a));

    // Start with all root nodes
    let mut order = root_nodes;

    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..order.len() {
            let element = order[i];
            if let Some(values) = dict.remove(&element) {
                order.splice(i + 1..i + 1, values.iter().cloned());
                changed = true;

                let mut seen = HashSet::new();
                let mut remove_indices = Vec::new();
                for j in (0..order.len()).rev() {
                    if !seen.insert(order[j]) {
                        remove_indices.push(j);
                    }
                }
                for idx in remove_indices.into_iter() {
                    order.remove(idx);
                }
                break;
            }
        }
    }

    order
}

fn solve1(correct_order: Vec<i32>, updates: Vec<Vec<i32>>) -> i32 {
    let mut sum_middle_pages = 0;

    for update in updates {
        let mut is_valid = true;
        let mut last_pos = -1;

        for &num in &update {
            if let Some(pos) = correct_order.iter().position(|&x| x == num) {
                if last_pos > pos as i32 {
                    is_valid = false;
                    break;
                }
                last_pos = pos as i32;
            }
        }

        if is_valid {
            let middle_idx = update.len() / 2;
            let middle_value = update[middle_idx];
            sum_middle_pages += middle_value;

            println!("Valid update: {:?}, middle value: {}", update, middle_value);
        } else {
            println!("Invalid update: {:?}", update);
        }
    }

    sum_middle_pages
}

fn main() {
    let mut dict = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    reader("example.txt".to_string(), &mut dict, &mut updates);
    println!("{:?}", dict);

    let mut sorted_dict: Vec<(i32, Vec<i32>)> = dict.into_iter().collect();
    sorted_dict.sort_by_key(|(key, _)| -key);
    let mut dict = sorted_dict.into_iter().collect();

    let correct_order = process(&mut dict);
    println!("{:?}", correct_order);

    let ans1 = solve1(correct_order, updates);
    println!("{}", ans1);
}
