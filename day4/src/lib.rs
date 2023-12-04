use std::collections::HashMap;

use common::Runner;

struct Part1;

impl Runner for Part1 {
    fn run(input: &str) -> u32 {
        let lines = input.split('\n').filter(|l| !l.is_empty());

        let mut sum = 0;
        for l in lines {
            let mut card = l.split(':');
            _ = card.next();
            let mut numbers = card.next().unwrap().split('|');
            let winning_nums = get_number_from_list(numbers.next().unwrap());
            let nums = get_number_from_list(numbers.next().unwrap());

            let mut winning_count = 0;
            winning_nums.iter().for_each(|w| {
                if nums.contains(w) {
                    winning_count += 1;
                }
            });

            if winning_count == 0 {
                continue;
            }

            let mut points = 1;
            for _ in 0..(winning_count - 1) {
                points *= 2;
            }
            sum += points;
        }

        sum
    }
}

struct Part2;

impl Runner for Part2 {
    fn run(input: &str) -> u32 {
        let lines = input.split('\n').filter(|l| !l.is_empty());

        let mut sum = 0;

        let mut final_cards: HashMap<u32, u32> = HashMap::new();
        for l in lines {
            let l = l.trim();
            let mut card = l.split(':').filter(|l| !l.is_empty());
            let mut card_num = card.next().unwrap().split(' ').filter(|c| !c.is_empty());
            card_num.next();
            let id = get_number(card_num.next().unwrap());
            final_cards.entry(id).and_modify(|e| *e += 1).or_insert(1);

            let mut numbers = card.next().unwrap().split('|');
            let winning_nums = get_number_from_list(numbers.next().unwrap());
            let nums = get_number_from_list(numbers.next().unwrap());

            let mut winning_count = 0;
            winning_nums.iter().for_each(|w| {
                if nums.contains(w) {
                    winning_count += 1;
                }
            });

            if winning_count == 0 {
                continue;
            }

            let count = final_cards.get(&id).unwrap();
            for _ in 0..*count {
                for c in (id + 1)..=(id + winning_count) {
                    final_cards.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }

        for (_, c) in final_cards {
            sum += c;
        }

        sum
    }
}

fn get_number_from_list(list: &str) -> Vec<u32> {
    let num_str = list.split(' ').filter(|n| !n.is_empty());

    let mut nums = Vec::new();
    for s in num_str {
        nums.push(get_number(s));
    }

    nums
}

fn get_number(n_str: &str) -> u32 {
    assert!(!n_str.is_empty());

    let mut num = 0;
    for c in n_str.chars() {
        num = (num * 10) + c.to_digit(10).unwrap();
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = Part1::run(input);
        assert_eq!(res, 13);
    }

    #[test]
    fn part1() {
        let input = include_str!("../input.txt");
        let res = Part1::run(input);
        assert_eq!(res, 19135);
    }

    #[test]
    fn part2_example() {
        let input = r"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = Part2::run(input);
        assert_eq!(res, 30);
    }

    #[test]
    fn part2() {
        let input = include_str!("../input.txt");
        let res = Part2::run(input);
        assert_eq!(res, 5704953);
    }
}
