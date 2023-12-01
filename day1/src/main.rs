use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../input2.txt").to_string();
    let sum = run(input)?;
    println!("{sum}");
    Ok(())
}

fn run(input: String) -> Result<u32> {
    let lines = input.split('\n');

    let mut sum = 0;

    for line in lines {
        let numbers = get_numbers(line);

        if let Some((f, s)) = numbers {
            sum += f * 10 + s;
        }
    }

    Ok(sum)
}

fn get_numbers(line: &str) -> Option<(u32, u32)> {
    let mut first = None;
    let mut second = None;

    let mut line = line;
    while let Some((i, d)) = contains_number(line) {
        if first.is_none() {
            first = Some(d);
        } else {
            second = Some(d);
        }
        line = &line[(i + 1)..line.len()];
    }

    first?;

    if second.is_none() {
        second = first;
    }

    Some((first.unwrap(), second.unwrap()))
}

fn contains_number(line: &str) -> Option<(usize, u32)> {
    let numbers: HashMap<String, u32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    let mut s = "".to_string();
    let mut it = line.chars().enumerate();
    let mut curr = it.next();

    while let Some((i, c)) = curr {
        if c.is_numeric() {
            return Some((i, c.to_digit(10).unwrap()));
        } else {
            s += &c.to_string();
            let start_of_num = numbers.keys().any(|number| number.starts_with(&s));
            if !start_of_num {
                s.remove(0);
            }
            // let start_with = numbers.keys().any(|n| n.starts_with(&(s + &c.to_string())));
            // if !s.is_empty() || start_with {
            // }

            if numbers.contains_key(&s.to_lowercase()) {
                return Some((i, numbers[&s]));
            }
        }

        curr = it.next();
    }

    None
}

mod tests {
    use super::*;

    #[test]
    fn contains_number_test() {
        #[derive(PartialEq, Eq, Debug)]
        struct Test {
            line: String,
            expected: Option<(usize, u32)>,
        }

        let tests: Vec<Test> = vec![
            Test {
                line: "9".to_string(),
                expected: Some((0, 9)),
            },
            Test {
                line: "Hello WOrld".to_string(),
                expected: None,
            },
            Test {
                line: "one".to_string(),
                expected: Some((2, 1)),
            },
            Test {
                line: "two".to_string(),
                expected: Some((2, 2)),
            },
            Test {
                line: "three".to_string(),
                expected: Some((4, 3)),
            },
        ];

        for test in tests {
            let actual = contains_number(&test.line);
            assert!(
                actual == test.expected,
                "Actual: {:?}, Test: {:?}",
                actual,
                test
            );
        }
    }

    #[test]
    fn words() {
        struct Test {
            input: String,
            expected: Option<(u32, u32)>,
        }

        let tests = vec![
            Test {
                input: "two1nine".to_string(),
                expected: Some((2, 9)),
            },
            Test {
                input: "eighttwothree".to_string(),
                expected: Some((8, 3)),
            },
            Test {
                input: "abcone2threexyz".to_string(),
                expected: Some((1, 3)),
            },
            Test {
                input: "xtwone3four".to_string(),
                expected: Some((2, 4)),
            },
            Test {
                input: "4nineeightseven2".to_string(),
                expected: Some((4, 2)),
            },
            Test {
                input: "zoneight234".to_string(),
                expected: Some((1, 4)),
            },
            Test {
                input: "7pqrstsixteen".to_string(),
                expected: Some((7, 6)),
            },
        ];

        for test in tests {
            let actual = get_numbers(&test.input);
            assert!(actual == test.expected);
        }
    }

    #[test]
    fn part1() {
        let input = include_str!("../input1.txt");
        let r = run(input.to_string());
        assert!(r.is_ok());
    }

    #[test]
    fn part2() {
        let input = include_str!("../input2.txt");
        let r = run(input.to_string());
        assert!(r.is_ok());
    }
}
