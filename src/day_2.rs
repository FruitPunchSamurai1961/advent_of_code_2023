use std::cmp::max;
use crate::helper as helper;

#[derive(Debug)]
struct SubGame {
    green: u32,
    blue: u32,
    red: u32,
}

impl SubGame {
    fn product(&self) -> u32 {
        return self.blue * self.green * self.red;
    }
}

#[derive(Debug)]
struct GameInfo {
    id: u32,
    sub_games: Vec<SubGame>,
}

pub fn solve(input: &str) {
    println!("Solving day 2 problems...");
    let lines = input.lines();
    let mut p_1_answer = 0;
    let max_poss_sub_game = SubGame {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut p_2_answer = 0;

    for line in lines {
        let game_info = parse_game_info(line);
        if is_game_possible(&game_info, &max_poss_sub_game) {
            p_1_answer += game_info.id;
        }
        let max_cubes_needed = get_max_cubes_needed_for_valid_game(&game_info);
        p_2_answer += max_cubes_needed.product();
    }

    println!("The answer to the first part of day 2 problem is: {}", p_1_answer);
    println!("The answer to the second part of day 2 problem is: {}", p_2_answer);
}

fn is_game_possible(curr_game: &GameInfo, max_poss_sub_game: &SubGame) -> bool {
    for sub_game in curr_game.sub_games.as_slice() {
        if sub_game.red > max_poss_sub_game.red || sub_game.green > max_poss_sub_game.green || sub_game.blue > max_poss_sub_game.blue {
            return false;
        }
    }
    true
}

fn get_max_cubes_needed_for_valid_game(curr_game: &GameInfo) -> SubGame {
    let mut max_cubes_sub_game: SubGame = SubGame {
        green: 0,
        blue: 0,
        red: 0,
    };

    for sub_game in curr_game.sub_games.as_slice() {
        max_cubes_sub_game.red = max(max_cubes_sub_game.red, sub_game.red);
        max_cubes_sub_game.blue = max(max_cubes_sub_game.blue, sub_game.blue);
        max_cubes_sub_game.green = max(max_cubes_sub_game.green, sub_game.green);
    }

    max_cubes_sub_game
}

fn parse_game_info(line: &str) -> GameInfo {
    let line_substr: Vec<_> = line.split(":").collect();
    let game_id: Vec<u32> = helper::filter_digits_from_str(line_substr[0]);
    let games: Vec<_> = line_substr[1].split(";").collect();
    let sub_games: Vec<SubGame> = parse_sub_games(games);
    GameInfo { id: helper::convert_num_arr_to_num(game_id), sub_games }
}

fn parse_sub_games(games: Vec<&str>) -> Vec<SubGame> {
    let mut sub_games: Vec<SubGame> = Vec::new();
    for game in games {
        let balls_info: Vec<_> = game.split(",").collect();
        let mut sub_game: SubGame = SubGame {
            green: 0,
            blue: 0,
            red: 0,
        };
        for ball in balls_info {
            let count_of_ball: Vec<u32> = helper::filter_digits_from_str(ball);
            let color_of_ball: String = ball.chars()
                .filter(|x| x.is_ascii_alphabetic())
                .collect();
            match color_of_ball.as_str() {
                "green" => sub_game.green = helper::convert_num_arr_to_num(count_of_ball),
                "blue" => sub_game.blue = helper::convert_num_arr_to_num(count_of_ball),
                "red" => sub_game.red = helper::convert_num_arr_to_num(count_of_ball),
                _ => unreachable!()
            }
        }
        sub_games.push(sub_game);
    }
    sub_games
}