use std::collections::HashSet;

static DIRS: [(i64, i64); 8] = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)];

pub fn solve(input: &str) {
    println!("Solving day 3 problems...");
    let lines = input.lines();
    let mut arr = Vec::new();
    for line in lines {
        let parsed_line_arr = parse_line(line);
        arr.push(parsed_line_arr);
    }
    println!("The solution to part one of day 3 is: {}", get_valid_machine_parts(&arr));
    println!("The solution to part two of day 3 is: {}", get_total_gear_ratio(&arr));
}

fn parse_line(line: &str) -> Vec<char> {
    let mut line_arr = Vec::new();
    for char in line.chars() {
        line_arr.push(char);
    }
    line_arr
}

fn get_valid_machine_parts(arr: &Vec<Vec<char>>) -> u64 {
    let mut answer: u64 = 0;
    let mut hash_set: HashSet<(usize, usize)> = HashSet::new();
    for (outer_idx, char_arr) in arr.iter().enumerate() {
        for (inner_idx, val) in char_arr.iter().enumerate() {
            if !hash_set.contains(&(outer_idx, inner_idx)) && val.is_digit(10) && has_neighbor_symbol(inner_idx as i64, outer_idx as i64, &arr) {
                answer += get_full_number(outer_idx, inner_idx, &arr[outer_idx], &mut hash_set);
            }
        }
    }
    answer
}

fn has_neighbor_symbol(inner_idx: i64, outer_idx: i64, arr: &Vec<Vec<char>>) -> bool {
    let n = arr.len() as i64;
    let m = arr[0].len() as i64;
    for (dx, dy) in DIRS {
        let x = outer_idx + dx;
        let y = inner_idx + dy;
        if 0 <= x && x < n && 0 <= y && y < m && !arr[x as usize][y as usize].is_digit(10) && !(arr[x as usize][y as usize] == '.') {
            return true;
        };
    };
    false
}

fn get_full_number(outer_idx: usize, inner_idx: usize, arr: &Vec<char>, hash_set: &mut HashSet<(usize, usize)>) -> u64 {
    let mut answer = 0;
    let mut start = inner_idx;

    while (start as i64 - 1) >= 0 && arr[start - 1].is_digit(10) {
        start -= 1;
    };


    let mut end = inner_idx;
    while end < arr.len() && arr[end].is_digit(10) {
        end += 1;
    };

    for idx in start..end {
        answer = answer * 10 + arr[idx].to_digit(10).unwrap() as u64;
        hash_set.insert((outer_idx, idx));
    }
    answer
}


fn get_total_gear_ratio(arr: &Vec<Vec<char>>) -> u64 {
    let mut answer: u64 = 0;
    for (outer_idx, char_arr) in arr.iter().enumerate() {
        for (inner_idx, val) in char_arr.iter().enumerate() {
            if !val.is_digit(10) && *val != '.' {
                answer += get_gear_ratio(inner_idx as i64, outer_idx as i64, &arr);
            }
        }
    }
    answer
}

fn get_gear_ratio(inner_idx: i64, outer_idx: i64, arr: &Vec<Vec<char>>) -> u64 {
    let n = arr.len() as i64;
    let m = arr[0].len() as i64;
    let mut hash_set: HashSet<(usize, usize)> = HashSet::new();
    let mut cnt = 0;
    let mut answer = 1;
    for (dx, dy) in DIRS {
        let x = outer_idx + dx;
        let y = inner_idx + dy;
        if 0 <= x && x < n && 0 <= y && y < m && !hash_set.contains(&(x as usize, y as usize)) && arr[x as usize][y as usize].is_digit(10) {
            cnt += 1;
            answer *= get_full_number(x as usize, y as usize, &arr[x as usize], &mut hash_set)
        };
    };
    if cnt == 2 {
        return answer;
    } else { 0 }
}