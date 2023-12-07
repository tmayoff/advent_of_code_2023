use common::{get_number_from_list, Runner};

struct Part1;
impl Runner for Part1 {
    fn run(input: &str) -> u64 {
        let mut lines = input.lines().map(|l| l.trim()).skip_while(|l| l.is_empty());

        let mut times = lines.next().unwrap().split(':');
        times.next().unwrap();
        let times = get_number_from_list(times.next().unwrap());

        let mut distances = lines.next().unwrap().split(':');
        distances.next().unwrap();
        let distances = get_number_from_list(distances.next().unwrap());

        let races = times.len();

        let mut count = 1;
        for r in 0..races {
            let race_length = *times.get(r).unwrap() as u64;
            let record = *distances.get(r).unwrap() as u64;

            let mut beat = Vec::new();
            for power_time in 0..race_length {
                let travel_time = race_length - power_time;
                let dst = travel_time * power_time;
                if dst > record {
                    beat.push(power_time);
                }
            }

            count *= beat.len() as u64;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = r"
        Time:      7  15   30
        Distance:  9  40  200";

        let result = Part1::run(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn part1() {
        let input = r"
        Time:        35     69     68     87
        Distance:   213   1168   1086   1248
        ";

        let result = Part1::run(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn part2_sample() {
        let input = r"
        Time:      71530
        Distance:  940200";

        let result = Part1::run(input);
        assert_eq!(result, 71503);
    }

    #[test]
    fn part2() {
        let input = r"
        Time:        35696887
        Distance:   213116810861248
        ";

        let result = Part1::run(input);
        assert_eq!(result, 20537782);
    }
}
