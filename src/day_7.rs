use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    FIVE = 1,
    FOUR = 2,
    FULL = 3,
    THREE = 4,
    TWO = 5,
    ONE = 6,
    HIGH = 7,
}

#[derive(Debug, Eq)]
struct CamelCard {
    label: String,
    label_strength: Vec<u64>,
    kind: Kind,
    bid: u64,
}


#[derive(Debug)]
struct CamelCards {
    camel_cards: Vec<CamelCard>,
}


impl Kind {
    fn get_kind_from_label(label: &str, is_part_two: bool) -> Self {
        let mut counts = [0; 13];
        for c in label.chars() {
            match c {
                'A' => counts[0] += 1,
                '2' => counts[1] += 1,
                '3' => counts[2] += 1,
                '4' => counts[3] += 1,
                '5' => counts[4] += 1,
                '6' => counts[5] += 1,
                '7' => counts[6] += 1,
                '8' => counts[7] += 1,
                '9' => counts[8] += 1,
                'T' => counts[9] += 1,
                'J' => counts[10] += 1,
                'Q' => counts[11] += 1,
                'K' => counts[12] += 1,
                _ => (),
            }
        }

        if is_part_two  && counts[10] > 0{
            let joker_count = counts[10];
            counts[10] = 0;
            let max_count_index = counts.iter().enumerate().max_by_key(|&(_, &count)| count).map(|(i, _)| i);
            if let Some(index) = max_count_index {
                counts[index] += joker_count;
            }
        }

        let unique_count = counts.iter().filter(|&x| *x > 0).count();

        match unique_count {
            1 => Kind::FIVE,
            2 => {
                if counts.iter().any(|&x| x == 4) {
                    Kind::FOUR
                } else {
                    Kind::FULL
                }
            }
            3 => {
                if counts.iter().any(|&x| x == 3) {
                    Kind::THREE
                } else {
                    Kind::TWO
                }
            }
            4 => Kind::ONE,
            _ => Kind::HIGH,
        }
    }
}


impl CamelCard {
    fn new(label: &str, bid: u64, is_part_two: bool) -> Self {
        CamelCard {
            label: String::from(label),
            label_strength: if is_part_two {
                CamelCard::get_label_strength_part_two(label)
            } else {
                CamelCard::get_label_strength_part_one(label)
            },
            kind: Kind::get_kind_from_label(label, is_part_two),
            bid,
        }
    }

    fn get_label_strength_part_one(label: &str) -> Vec<u64> {
        label.chars()
            .map(|c| match c {
                'A' => 1,
                'K' => 2,
                'Q' => 3,
                'J' => 4,
                'T' => 5,
                '9' => 6,
                '8' => 7,
                '7' => 8,
                '6' => 9,
                '5' => 10,
                '4' => 11,
                '3' => 12,
                '2' => 13,
                _ => 0,
            })
            .collect()
    }

    fn get_label_strength_part_two(label: &str) -> Vec<u64> {
        label.chars()
            .map(|c| match c {
                'A' => 1,
                'K' => 2,
                'Q' => 3,
                'T' => 4,
                '9' => 5,
                '8' => 6,
                '7' => 7,
                '6' => 8,
                '5' => 9,
                '4' => 10,
                '3' => 11,
                '2' => 12,
                'J' => 13,
                _ => 0,
            })
            .collect()
    }
}

impl PartialEq<Self> for CamelCard {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.bid == other.bid && self.label == other.label
    }
}

impl PartialOrd<Self> for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.kind.cmp(&other.kind) {
            Ordering::Less => { Some(Ordering::Less) }
            Ordering::Equal => {
                for (self_strength, other_strength) in self.label_strength.iter().zip(other.label_strength.iter()) {
                    match self_strength.cmp(other_strength) {
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Equal => continue,
                    }
                }
                Some(self.bid.cmp(&other.bid))
            }
            Ordering::Greater => { Some(Ordering::Greater) }
        }
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl CamelCards {
    fn new() -> Self {
        CamelCards { camel_cards: Vec::new() }
    }

    fn insert(&mut self, label: &str, bid: u64, is_part_two: bool) {
        self.camel_cards.push(CamelCard::new(label, bid, is_part_two));
    }

    fn sort(&mut self) {
        self.camel_cards.sort();
    }
}

pub fn solve(input: &str) {
    println!("Solving Day 7 problems...");
    let mut part_one_cards = parse_input(input, false);
    let part_one_solution = get_ranks_bid_summation(&mut part_one_cards);

    let mut part_two_cards = parse_input(input, true);
    let part_two_solution = get_ranks_bid_summation(&mut part_two_cards);

    println!("The answer to the first gold star for day 7 is: {}", part_one_solution);
    println!("The answer to the second gold star for day 7 is: {}", part_two_solution);
}

fn parse_input(input: &str, is_part_two: bool) -> CamelCards {
    let mut cards = CamelCards::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(label), Some(bid_str)) = (parts.next(), parts.next()) {
            if let Ok(bid) = bid_str.parse::<u64>() {
                cards.insert(label, bid, is_part_two);
            }
        }
    }

    cards
}

fn get_ranks_bid_summation(camel_cards: &mut CamelCards) -> u64 {
    camel_cards.sort();
    let mut answer = 0;
    let mut rank = 1;
    for card in camel_cards.camel_cards.iter().rev() {
        answer += rank * card.bid;
        rank += 1;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut camel_cards = CamelCards::new();
        camel_cards.insert("32T3K", 765, true);
        camel_cards.insert("T55J5", 684, true);
        camel_cards.insert("KK677", 28, true);
        camel_cards.insert("KTJJT", 220, true);
        camel_cards.insert("QQQJA", 483, true);

        let result = get_ranks_bid_summation(&mut camel_cards);
        assert_eq!(result, 5905);

        // You can add more assertions for other test cases here if needed
    }
}
