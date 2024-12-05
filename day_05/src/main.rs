use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn reader(input_file: String, dict: &mut HashSet<(i32, i32)>, updates: &mut Vec<Vec<i32>>) {
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
        dict.insert((key, value));
    }

    for line in &lines[split_index + 1..] {
        updates.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
    }
}
fn solve1(dict: &HashSet<(i32, i32)>, updates: Vec<Vec<i32>>) -> i32 {
    let mut sum_middle_pages = 0;

    for update in updates {
        let mut is_valid = true;

        let update_positions: HashMap<i32, usize> = update
            .iter()
            .enumerate()
            .map(|(idx, &page)| (page, idx))
            .collect();

        let update_pages: HashSet<i32> = update.iter().cloned().collect();

        for &(u, v) in dict {
            if update_pages.contains(&u) && update_pages.contains(&v) {
                let pos_u = update_positions[&u];
                let pos_v = update_positions[&v];
                if pos_u >= pos_v {
                    is_valid = false;
                    break;
                }
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
    let mut dict = HashSet::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    reader("example.txt".to_string(), &mut dict, &mut updates);
    println!("{:?}", dict);

    let ans1 = solve1(&dict, updates);
    println!("{}", ans1);
}
