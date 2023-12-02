use std::collections::HashMap;

use common::Runner;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {}

pub struct Part1;
impl Runner for Part1 {
    fn run(input: &str) -> u32 {
        let lines = input.split('\n').filter(|l| !l.is_empty());

        let mut sum = 0;
        for line in lines {
            let id = get_game_id(line);
            let pulls = Part1::get_pulls(line);
            let mut valid = true;
            for (col, n) in pulls {
                let max = match col.as_str() {
                    "red" => MAX_RED,
                    "green" => MAX_GREEN,
                    "blue" => MAX_BLUE,
                    _ => panic!("Unknown Color"),
                };

                if n > max {
                    valid = false;
                }
            }

            if valid {
                sum += id;
            }
        }

        sum
    }
}

impl Part1 {
    fn get_pulls(line: &str) -> HashMap<String, u32> {
        let mut pull = HashMap::new();

        let mut line = line.split(':');
        line.next();
        let pulls = line.next().unwrap().split(';');
        for p in pulls {
            let cols = p.split(',');

            for col in cols {
                let mut col = col.split(' ').filter(|c| !c.is_empty());
                let n = col.next().unwrap();
                let n = get_number(n);
                let col = col.next().unwrap();

                if pull.contains_key(col) {
                    if pull.get(col).unwrap() < &n {
                        *pull.get_mut(col).unwrap() = n;
                    }
                } else {
                    pull.insert(col.to_string(), n);
                }
            }
        }

        pull
    }
}

struct Part2;
impl Runner for Part2 {
    fn run(input: &str) -> u32 {
        let lines = input.split('\n').filter(|l| !l.is_empty());

        let mut sum = 0;
        for line in lines {
            let pulls = Part2::get_pulls(line);
            let mut power = 1;
            for (_, p) in pulls {
                power *= p;
            }

            sum += power;
        }

        sum
    }
}

impl Part2 {
    fn get_pulls(line: &str) -> HashMap<String, u32> {
        let mut pull = HashMap::new();

        let mut line = line.split(':');
        line.next();
        let pulls = line.next().unwrap().split(';');
        for p in pulls {
            let cols = p.split(',');

            for col in cols {
                let mut col = col.split(' ').filter(|c| !c.is_empty());
                let n = col.next().unwrap();
                let n = get_number(n);
                let col = col.next().unwrap();

                if pull.contains_key(col) {
                    if pull.get(col).unwrap() < &n {
                        *pull.get_mut(col).unwrap() = n;
                    }
                } else {
                    pull.insert(col.to_string(), n);
                }
            }
        }

        pull
    }
}

fn get_game_id(line: &str) -> u32 {
    let mut game = line.split(':').next().unwrap().split(' ');
    game.next();
    let id = game.next().unwrap();

    get_number(id)
}

fn get_number(n_str: &str) -> u32 {
    let mut num = 0;
    for c in n_str.chars() {
        num = (num * 10) + c.to_digit(10).unwrap();
    }

    num
}

mod tests {
    use common::Runner;

    #[test]
    fn get_game_id() {
        let r = super::get_game_id("Game 1: asdsad");
        assert_eq!(r, 1);
    }

    #[test]
    fn get_pull() {
        let pull =
            super::Part2::get_pulls("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert!(pull.contains_key("red"));
        assert!(pull.contains_key("green"));
        assert!(pull.contains_key("blue"));

        assert_eq!(pull.get("red").unwrap(), &4);
        assert_eq!(pull.get("green").unwrap(), &2);
        assert_eq!(pull.get("blue").unwrap(), &6);
    }

    #[test]
    fn part1() {
        let input = include_str!("../input.txt");
        let res = super::Part1::run(input);
        assert_eq!(res, 1734);
    }

    #[test]
    fn part2() {
        let input = include_str!("../input.txt");
        let res = super::Part2::run(input);
        assert_eq!(res, 70387);
    }
}
