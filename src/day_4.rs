use std::collections::{HashMap, HashSet};

use crate::helper as helper;

#[derive(Debug)]
struct CardGame {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers_in_possession: HashSet<u32>,
}

impl CardGame {
    fn calculate_points(&self) -> u32 {
        let matching = self.winning_numbers.intersection(&self.numbers_in_possession).count() as u32;
        if matching > 0 {
            1 << (matching - 1)
        } else {
            0
        }
    }

    fn calculate_scratch_cards_won(&self) -> u32 {
        self.winning_numbers.intersection(&self.numbers_in_possession).count() as u32
    }
}

pub fn solve(input: &str) {
    println!("Solving day 4 problems...");
    let lines = input.lines();
    let mut cards: Vec<CardGame> = Vec::new();
    let mut p_1_answer = 0;
    let mut p_2_answer = 0;
    let mut scratch_cards_map: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        cards.push(parse_line_for_card_game(line));
    }
    for card in cards.iter() {
        p_1_answer += card.calculate_points()
    }

    cards.reverse();
    for card in cards {
        p_2_answer+=1;
        let mut won = card.calculate_scratch_cards_won();
        let max_card_id = card.id+won;
        for next_card_id in card.id+1..=max_card_id {
            won += scratch_cards_map.get(&next_card_id).unwrap_or(&0);
        }
        scratch_cards_map.insert(card.id, won);
        p_2_answer+=won;
    }


    println!("The solution to part one of day 3 is: {}", p_1_answer);
    println!("The solution to part two of day 3 is: {}", p_2_answer);
}

fn parse_line_for_card_game(line: &str) -> CardGame {
    let line_substr: Vec<_> = line.split(":").collect();
    let game_id: Vec<u32> = helper::filter_digits_from_str(line_substr[0]);
    let games: Vec<_> = line_substr[1].split("|").collect();
    let mut winning_numbers = HashSet::new();
    let mut numbers_in_possession = HashSet::new();
    for num in helper::get_num_arr_from_space_seperated_str(games[0]) {
        winning_numbers.insert(num);
    }
    for num in helper::get_num_arr_from_space_seperated_str(games[1]) {
        numbers_in_possession.insert(num);
    }
    CardGame {
        id: helper::convert_num_arr_to_num(game_id),
        winning_numbers,
        numbers_in_possession,
    }
}