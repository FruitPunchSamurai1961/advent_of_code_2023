use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'd', long, value_delimiter = ',', num_args = 1.., help = "Takes in a comma seperated list of integers representing the days to test")]
    days: Vec<i32>,
}


fn main() {
    let args = Args::parse();

    let days;

    if !args.days.is_empty() {
        days = args.days;
    } else {
        days = (1..=25).collect();
    }

    for day in &days {
        println!("Day {}", day);
        let input_file_path = format!("./data/day_{}.txt", day);
        let data = fs::read_to_string(&input_file_path);
        if let Ok(data) = data {
            let data = data.trim();
            let func_to_run = match day {
                1 => advent_of_code_2023::day_1::solve,
                2 => advent_of_code_2023::day_2::solve,
                _ => unreachable!(),
            };
            func_to_run(data);
            println!();
        }
    }
}
