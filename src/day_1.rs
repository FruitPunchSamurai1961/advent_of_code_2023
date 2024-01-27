use std::cmp::{max, min};
use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Solving day 1 problems...");
    let strings = input.lines();
    let mut part_one_answer: u32 = 0;
    let mut part_two_answer: u32 = 0;
    for word in strings {
        part_one_answer += extract_numeric_digits(word);
        part_two_answer += parse_number_from_word(word);
    }
    println!("The answer is to the first gold star for day 1 is: {}", part_one_answer);
    println!("The answer is to the second gold star for day 1 is: {}", part_two_answer);
}


fn extract_numeric_digits(word: &str) -> u32 {
    let chars: Vec<_> = word.chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    chars[0] * 10 + chars[chars.len() - 1]
}


fn parse_number_from_word(word: &str) -> u32 {
    let pattern_vec = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut number_to_idx_mapping = HashMap::new();
    let mut min_index = word.len();
    let mut max_index = 0usize;

    for pattern in pattern_vec {
        let indices: Vec<_> = word.match_indices(pattern).collect();
        if !indices.is_empty() {
            let curr_min_index = *indices.iter().min().unwrap();
            let curr_max_index = *indices.iter().max().unwrap();
            number_to_idx_mapping.insert(curr_max_index.0, pattern);
            number_to_idx_mapping.insert(curr_min_index.0, pattern);
            min_index = min(min_index, curr_min_index.0);
            max_index = max(max_index, curr_max_index.0);
        }
    }

    let first_digit = get_int_from_string_mapping(number_to_idx_mapping.get(&min_index).unwrap());
    let second_digit = get_int_from_string_mapping(number_to_idx_mapping.get(&max_index).unwrap());
    first_digit * 10 + second_digit
}

fn get_int_from_string_mapping(str_num: &str) -> u32 {
    match str_num {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => unreachable!()
    }
}