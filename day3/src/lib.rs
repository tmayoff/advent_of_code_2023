use std::{collections::HashSet, ops::RangeInclusive};

struct Part1;

impl common::Runner for Part1 {
    fn run(input: &str) -> u32 {
        let lines = input.split('\n').filter(|l| !l.is_empty());

        let mut grid = Vec::new();

        for line in lines {
            let mut row = Vec::new();
            for c in line.trim().chars() {
                row.push(c);
            }

            grid.push(row);
        }

        let mut ranges = Vec::new();

        for y in 0..grid.len() {
            let row = grid.get(y).unwrap();

            let mut adjacent = false;
            let mut start = None;
            for x in 0..=row.len() {
                let cell = grid.get(y).unwrap().get(x);
                let mut end = cell.is_none();

                if let Some(c) = cell {
                    if c.is_numeric() {
                        if start.is_none() {
                            start = Some(x);
                        }

                        if !adjacent {
                            let neihbours = Part1::get_neighbours(&grid, (x, y));
                            adjacent = neihbours.iter().any(|c| !c.is_numeric() && c != &'.');
                        }
                    } else {
                        end = true;
                    }
                }

                if end && start.is_some() {
                    if adjacent {
                        ranges.push((y, start.unwrap()..x));
                    }

                    adjacent = false;
                    start = None;
                }
            }
        }

        let mut part_numbers = Vec::new();
        for (y, range) in ranges {
            let row = grid.get(y).unwrap();

            let mut num = 0;
            for c in range {
                let c = row.get(c).unwrap();
                let n = c.to_digit(10);
                if let Some(n) = n {
                    num = (num * 10) + n;
                }
            }

            part_numbers.push(num);
        }

        let mut sum = 0;
        part_numbers.iter().for_each(|n| sum += n);

        sum
    }
}

impl Part1 {
    fn get_neighbours(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<char> {
        let mut neighbours = Vec::new();
        let height = grid.len() as i32;
        let width = grid.get(0).unwrap().len() as i32;

        let offsets: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for offset in offsets {
            let x = pos.0 as i32;
            let y = pos.1 as i32;
            let offset_pos = (x + offset.0, y + offset.1);

            if offset_pos.0 < 0
                || offset_pos.1 < 0
                || offset_pos.0 >= width
                || offset_pos.1 >= height
            {
                continue;
            }

            let y = offset_pos.1 as usize;
            let x = offset_pos.0 as usize;

            neighbours.push(grid.get(y).unwrap().get(x).unwrap().to_owned());
        }

        neighbours
    }
}

struct Part2;

impl common::Runner for Part2 {
    fn run(input: &str) -> u32 {
        let lines = input.split('\n').filter(|l| !l.is_empty());

        let mut grid = Vec::new();

        for line in lines {
            let mut row = Vec::new();
            for c in line.trim().chars() {
                row.push(c);
            }

            grid.push(row);
        }

        let mut ratios = Vec::new();

        for y in 0..grid.len() {
            let row = grid.get(y).unwrap();

            for x in 0..=row.len() {
                let cell = grid.get(y).unwrap().get(x);

                if let Some(c) = cell {
                    if c == &'*' {
                        let mut adjacent_parts = HashSet::new();
                        let neighbours = Part2::get_neighbours(&grid, (x, y));
                        for (x, y, c) in neighbours {
                            if c.is_numeric() {
                                let num_range = Part2::get_number_range(&grid, (x, y));
                                adjacent_parts.insert((y, num_range));
                            }
                        }

                        if adjacent_parts.len() == 2 {
                            let mut gear_ratio = 1;
                            adjacent_parts.iter().for_each(|(y, r)| {
                                gear_ratio *= Part2::get_number(&grid, y.to_owned(), r.to_owned());
                            });

                            ratios.push(gear_ratio);
                        }
                    }
                }
            }
        }

        let mut sum = 0;
        ratios.iter().for_each(|n| sum += n);

        sum
    }
}

impl Part2 {
    fn get_number_range(grid: &[Vec<char>], pos: (usize, usize)) -> RangeInclusive<usize> {
        let mut start = None;
        let mut end = None;

        let row = grid.get(pos.1).unwrap();
        let mut e = 1;
        let x = pos.0 as i32;
        loop {
            if start.is_none() {
                let left = x - e;
                let l = row.get(left as usize);
                if let Some(l) = l {
                    if !l.is_numeric() {
                        start = Some(left + 1);
                    }
                } else {
                    start = Some(left + 1);
                }
            }

            if end.is_none() {
                let right = x + e;
                let r = row.get(right as usize);
                if let Some(r) = r {
                    if !r.is_numeric() {
                        end = Some(right - 1);
                    }
                } else {
                    end = Some(right - 1);
                }
            }

            if start.is_some() && end.is_some() {
                break;
            }

            e += 1;
        }

        (start.unwrap() as usize)..=(end.unwrap() as usize)
    }

    fn get_number(grid: &[Vec<char>], y: usize, range: RangeInclusive<usize>) -> u32 {
        let row = grid.get(y).unwrap();

        let mut num = 0;
        for x in range {
            num = (num * 10) + row.get(x).unwrap().to_digit(10).unwrap();
        }

        num
    }

    fn get_neighbours(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize, char)> {
        let mut neighbours = Vec::new();
        let height = grid.len() as i32;
        let width = grid.get(0).unwrap().len() as i32;

        let offsets: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for offset in offsets {
            let x = pos.0 as i32;
            let y = pos.1 as i32;
            let offset_pos = (x + offset.0, y + offset.1);

            if offset_pos.0 < 0
                || offset_pos.1 < 0
                || offset_pos.0 >= width
                || offset_pos.1 >= height
            {
                continue;
            }

            let y = offset_pos.1 as usize;
            let x = offset_pos.0 as usize;

            neighbours.push((x, y, grid.get(y).unwrap().get(x).unwrap().to_owned()));
        }

        neighbours
    }
}

#[cfg(test)]
mod tests {
    use common::Runner;

    #[test]
    fn example_part1() {
        let input = r"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let res = super::Part1::run(input);
        assert_eq!(res, 4361);
    }

    #[test]
    fn example_part2() {
        let input = r"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let res = super::Part2::run(input);
        assert_eq!(res, 467835);
    }

    #[test]
    pub fn part1() {
        let res = super::Part1::run(include_str!("../input.txt"));
        assert_eq!(res, 546312);
    }

    #[test]
    pub fn part2() {
        let res = super::Part2::run(include_str!("../input.txt"));
        assert_eq!(res, 87449461);
    }
}
