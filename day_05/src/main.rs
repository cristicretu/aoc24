use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn reader(input_file: String, dict: &mut HashMap<i32, Vec<i32>>, updates: &mut Vec<Vec<i32>>) {
    let file = File::open(&input_file).expect("Failed to open input file");
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read lines");

    // Find the index where the ordering rules end and updates begin
    let split_index = lines
        .iter()
        .position(|line| line.trim().is_empty())
        .unwrap_or(lines.len());

    // Parse the ordering rules
    for line in &lines[..split_index] {
        let mut split = line.split("|");
        let key = split
            .next()
            .expect("Missing left page in rule")
            .parse::<i32>()
            .expect("Invalid page number");
        let value = split
            .next()
            .expect("Missing right page in rule")
            .parse::<i32>()
            .expect("Invalid page number");
        dict.entry(key).or_insert(Vec::new()).push(value);
    }

    // Parse the updates
    for line in &lines[split_index + 1..] {
        if !line.trim().is_empty() {
            let update = line
                .split(",")
                .map(|x| x.parse::<i32>().expect("Invalid page number in update"))
                .collect::<Vec<_>>();
            updates.push(update);
        }
    }
}

fn process(dict: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut order = Vec::new();
    let mut visited = HashSet::new();

    // Collect all pages involved in the rules
    let mut all_pages = HashSet::new();
    for (&key, values) in dict.iter() {
        all_pages.insert(key);
        for &v in values {
            all_pages.insert(v);
        }
    }

    // Recursive DFS function
    fn dfs(
        node: i32,
        dict: &HashMap<i32, Vec<i32>>,
        visited: &mut HashSet<i32>,
        order: &mut Vec<i32>,
    ) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        if let Some(values) = dict.get(&node) {
            for &v in values {
                dfs(v, dict, visited, order);
            }
        }
        order.push(node);
    }

    // Perform DFS for each page
    for &page in &all_pages {
        dfs(page, dict, &mut visited, &mut order);
    }

    // Reverse to get the correct ordering
    order.reverse();
    order
}
/// Determines which updates are valid and calculates the sum of their middle page numbers.
fn solve1(order: &Vec<i32>, updates: &Vec<Vec<i32>>) -> i32 {
    // Map each page to its position in the global ordering
    let position_map: HashMap<i32, usize> = order
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    let mut sum_middle_pages = 0;

    for update in updates {
        let mut is_valid = true;
        let mut last_position = 0;

        for &page in update {
            if let Some(&pos) = position_map.get(&page) {
                if pos < last_position {
                    is_valid = false;
                    break;
                }
                last_position = pos;
            } else {
                // If page not in ordering, treat it as coming last
                last_position = usize::MAX;
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
    reader("1.txt".to_string(), &mut dict, &mut updates);

    let correct_order = process(&dict);
    println!("Global ordering: {:?}", correct_order);

    let ans1 = solve1(&correct_order, &updates);
    println!("Part 1: Sum of middle page numbers: {}", ans1);
}
