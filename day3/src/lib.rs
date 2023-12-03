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

        let height = grid.len();
        let width = grid.get(0).unwrap().len();

        // TODO group cells first

        let mut ranges = Vec::new();

        for y in 0..grid.len() {
            let row = grid.get(y).unwrap();

            let mut adjacent = false;
            let mut start = None;
            let mut end = None;
            for x in 0..row.len() {
                let cell = grid.get(y).unwrap().get(x).unwrap();

                if cell.is_numeric() {
                    if start.is_none() {
                        start = Some(x);
                    }
                    let neihbours = get_neighbours(&grid, (x, y));
                    adjacent = neihbours.iter().any(|c| !c.is_numeric() && c != &'.');
                } else if start.is_some() {
                    end = Some(x);

                    if adjacent {
                        ranges.push((y, start.unwrap()..end.unwrap(), adjacent));
                    }

                    adjacent = false;
                    start = None;
                    end = None;
                }
            }
        }

        let mut part_numbers = Vec::new();
        for (y, range, adjacent) in ranges {
            // Find number string
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

        if offset_pos.0 < 0 || offset_pos.1 < 0 || offset_pos.0 >= width || offset_pos.1 >= height {
            continue;
        }

        let y = offset_pos.1 as usize;
        let x = offset_pos.0 as usize;

        neighbours.push(grid.get(y).unwrap().get(x).unwrap().to_owned());
    }

    neighbours
}

mod tests {
    use common::Runner;

    #[test]
    fn example() {
        let input = r"
        467..114..
        *..*......
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
    pub fn part1() {
        let res = super::Part1::run(include_str!("../input.txt"));
        assert_eq!(res, 332970);
    }
}
