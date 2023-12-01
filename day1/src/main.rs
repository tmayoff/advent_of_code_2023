use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../input.txt").to_string();

    let lines = input.split('\n');

    let mut sum = 0;

    for line in lines {
        let mut first = None;
        let mut second = None;
        for c in line.chars() {
            if c.is_numeric() {
                if first.is_none() {
                    first = Some(c);
                } else {
                    second = Some(c);
                }
            }
        }

        if first.is_none() {
            continue;
        }

        if second.is_none() {
            second = first;
        }

        let first = first.unwrap().to_digit(10).unwrap();
        let second = second.unwrap().to_digit(10).unwrap();

        sum += first * 10 + second;
    }

    println!("{sum}");

    Ok(())
}
