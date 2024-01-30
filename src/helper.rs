pub(crate) fn filter_digits_from_str(string: &str) -> Vec<u32> {
    string.chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

pub(crate) fn get_num_arr_from_space_seperated_str(string: &str) -> Vec<u32> {
    let mut num_arr = Vec::new();
    let str_arr: Vec<_> = string.split(' ').collect();
    for s_num in str_arr {
        if s_num.parse::<u32>().is_ok() {
            num_arr.push(s_num.parse::<u32>().unwrap())
        }
    }
    num_arr
}

pub(crate) fn convert_num_arr_to_num(arr: Vec<u32>) -> u32 {
    let mut num = 0;
    for curr in arr {
        num = num * 10 + curr
    }
    return num;
}