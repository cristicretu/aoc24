use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DX: [i32; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];
const DY: [i32; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];

fn reader(input_file: String, mat: &mut Vec<Vec<char>>) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let ln = line.unwrap();
        mat.push(ln.trim().chars().collect());
    }
}

const MAPPING: [char; 4] = ['X', 'M', 'A', 'S'];

fn check_pattern(mat: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut ans = 0;
    let step = 0;

    if mat[x as usize][y as usize] != MAPPING[step] {
        return 0;
    }

    for k in 0..8 {
        let mut ok = true;
        let mut curr_step = step;
        for i in 0..4 {
            let nx = x + i * DX[k];
            let ny = y + i * DY[k];
            if nx < 0 || nx >= mat.len() as i32 || ny < 0 || ny >= mat[nx as usize].len() as i32 {
                ok = false;
                break;
            }
            if mat[nx as usize][ny as usize] != MAPPING[curr_step] {
                ok = false;
                break;
            }
            curr_step += 1;
        }
        if ok {
            ans += 1;
        }
    }

    ans
}

fn check_inbounds(mat: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x < 0 || x >= mat.len() as i32 || y < 0 || y >= mat[x as usize].len() as i32
}

fn check_xmas(mat: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if mat[x as usize][y as usize] != 'A' {
        return false;
    }

    let m_position_top = (x - 1, y - 1);
    let m_position_bottom = (x + 1, y - 1);
    let s_position_top = (x - 1, y + 1);
    let s_position_bottom = (x + 1, y + 1);

    if check_inbounds(mat, m_position_top.0, m_position_top.1)
        || check_inbounds(mat, m_position_bottom.0, m_position_bottom.1)
        || check_inbounds(mat, s_position_top.0, s_position_top.1)
        || check_inbounds(mat, s_position_bottom.0, s_position_bottom.1)
    {
        return false;
    }

    let word_diag_1 = vec![
        mat[m_position_top.0 as usize][m_position_top.1 as usize],
        mat[x as usize][y as usize],
        mat[s_position_bottom.0 as usize][s_position_bottom.1 as usize],
    ];
    let word_diag_2 = vec![
        mat[s_position_top.0 as usize][s_position_top.1 as usize],
        mat[x as usize][y as usize],
        mat[m_position_bottom.0 as usize][m_position_bottom.1 as usize],
    ];

    if (word_diag_1 == ['M', 'A', 'S'] || word_diag_1 == ['S', 'A', 'M'])
        && (word_diag_2 == ['M', 'A', 'S'] || word_diag_2 == ['S', 'A', 'M'])
    {
        return true;
    }

    false
}

fn solve1(mat: &Vec<Vec<char>>) -> i32 {
    let mut ans = 0;
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            ans += check_pattern(mat, i as i32, j as i32);
        }
    }
    ans
}

fn solve2(mat: &Vec<Vec<char>>) -> i32 {
    let mut ans = 0;
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if check_xmas(mat, i as i32, j as i32) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let mut mat: Vec<Vec<char>> = Vec::new();
    reader("1.txt".to_string(), &mut mat);

    let ans = solve1(&mat);
    println!("Part 1: {}", ans);

    let ans = solve2(&mat);
    println!("Part 2: {}", ans);
}
