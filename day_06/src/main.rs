use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn get_dir(c: char) -> (i32, i32) {
    match c {
        'N' | '^' => (-1, 0), // North
        'E' | '>' => (0, 1),  // East
        'S' | 'v' => (1, 0),  // South
        'W' | '<' => (0, -1), // West
        _ => (0, 0),          // Default case
    }
}

fn reader(input_file: String, mat: &mut Vec<Vec<char>>) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let ln = line.unwrap();
        mat.push(ln.trim().chars().collect());
    }
}

fn check_inbounds(mat: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x >= 0 && x < mat.len() as i32 && y >= 0 && y < mat[x as usize].len() as i32
}

fn rotate90degs(dir: char) -> char {
    match dir {
        'N' => 'E',
        'E' => 'S',
        'S' => 'W',
        'W' => 'N',
        _ => 'N',
    }
}

fn solve1(mat: &mut Vec<Vec<char>>) -> i32 {
    let mut current_pos: (i32, i32) = (0, 0);
    let mut current_dir = 'N';
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == '^' {
                current_pos = (i as i32, j as i32);
            }
        }
    }

    while check_inbounds(mat, current_pos.0, current_pos.1) {
        let (dx, dy) = get_dir(current_dir);
        let new_pos = (current_pos.0 + dx, current_pos.1 + dy);

        if !check_inbounds(mat, new_pos.0, new_pos.1) {
            break;
        }

        mat[current_pos.0 as usize][current_pos.1 as usize] = 'X';
        if mat[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            current_dir = rotate90degs(current_dir);
        } else {
            current_pos = new_pos;
            visited.insert(current_pos);
        }
    }

    visited.len() as i32
}

fn main() {
    let mut mat: Vec<Vec<char>> = Vec::new();

    reader("1.txt".to_string(), &mut mat);

    let ans = solve1(&mut mat);

    println!("Part 1: {}", ans);

    // for i in 0..mat.len() {
    //     for j in 0..mat[i].len() {
    //         print!("{}", mat[i][j]);
    //     }
    //     println!();
    // }
}
