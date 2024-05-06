#[derive(Debug, Clone)]
struct Race {
    total_time: u64,
    record_distance: u64,
}

impl Race {
    fn count_ways_to_surpass_record(&self) -> u64 {
        let mut count = 0;

        for prep_time in 1..self.total_time {
            let mut curr: i64 = (self.total_time * prep_time) as i64;
            curr -= prep_time.pow(2) as i64;
            curr -= self.record_distance as i64;
            if curr > 0 {
                count += 1;
            }
        }
        count
    }
}


pub fn solve(input: &str) {
    println!("Solving Day 6 problems...");
    let part_one_races = get_part_one_races(&input);
    let part_two_races = get_part_two_race(&input);


    let part_one_answer = solve_part_one(&part_one_races);
    let part_two_answer = solve_part_two(&part_two_races);

    println!("The answer to the first gold star for day 6 is: {}", part_one_answer);
    println!("The answer to the second gold star for day 6 is: {}", part_two_answer);
}


fn get_part_one_races(input: &str) -> Vec<Race> {
    let mut races = Vec::new();

    let lines: Vec<&str> = input.trim().split('\n').collect();

    let times: Vec<u64> = lines[0]
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<u64> = lines[1]
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        races.push(Race {
            total_time: time,
            record_distance: distance,
        });
    }

    races
}

fn get_part_two_race(input: &str) -> Race {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let time: u64 = lines[0]
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap().to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: u64 = lines[1]
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap().to_string())
        .collect::<String>()
        .parse()
        .unwrap();


    Race {
        record_distance: distance,
        total_time: time,
    }
}


fn solve_part_one(races: &Vec<Race>) -> u64 {
    let mut answer = 1;
    for race in races.iter() {
        answer *= race.count_ways_to_surpass_record();
    }
    answer
}


fn solve_part_two(race: &Race) -> u64 {
    race.count_ways_to_surpass_record()
}