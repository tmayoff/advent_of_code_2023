use rayon::prelude::*;
use std::{
    collections::HashMap,
    ops::Range,
    sync::{Arc, Mutex},
};

use common::{get_number, get_number_from_list, Runner};

struct Part1;
impl Runner for Part1 {
    fn run(input: &str) -> u32 {
        let mut lines = input
            .split('\n')
            .map(|l| l.trim())
            .skip_while(|l| l.is_empty());
        let seeds = lines.next().unwrap();
        let seeds = get_seeds(seeds);

        let mut lines = lines.clone().skip_while(|l| l.is_empty());

        let mut maps = HashMap::new();
        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let chain = &get_chain_maps("location", &maps);

        let mut min = None;
        for s in seeds {
            let loc = get_dst_num("seed", s, chain, &maps);
            if let Some(m) = min {
                if loc < m {
                    min = Some(loc);
                }
            } else {
                min = Some(loc);
            }
        }

        min.unwrap() as u32
    }
}

struct Part2;
impl Runner for Part2 {
    fn run(input: &str) -> u32 {
        let mut lines = input
            .split('\n')
            .map(|l| l.trim())
            .skip_while(|l| l.is_empty());
        let seeds = lines.next().unwrap();
        let seeds = get_seed_ranges(seeds);

        let mut lines = lines.clone().skip_while(|l| l.is_empty());

        let mut maps = HashMap::new();
        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let map_category = get_map_src_dst(lines.next().unwrap());
        let map = get_maps(&mut lines);
        maps.insert(map_category, map);

        let chain = &get_chain_maps("location", &maps);

        let min = Arc::new(Mutex::new(std::u64::MAX));
        seeds.par_iter().for_each(|r| {
            for s in r.clone() {
                let loc = get_dst_num("seed", s, chain, &maps);
                let mut min = min.lock().unwrap();

                *min = (*min).min(loc);
            }
        });

        let m = *min.lock().unwrap();
        m as u32
    }
}

fn get_dst_num(
    map_src: &str,
    input: u64,
    chain: &[String],
    maps: &HashMap<(&str, &str), Vec<(Range<u64>, Range<u64>)>>,
) -> u64 {
    let mut src = map_src;
    let mut input = input;

    for dst in chain {
        let map = maps.get(&(src, dst));
        if let Some(map) = map {
            input = get_dst_in_map(&input, map);

            src = dst;
        }
    }

    input
}

fn get_chain_maps<'a>(
    output_s: &str,
    maps: &HashMap<(&str, &str), Vec<(Range<u64>, Range<u64>)>>,
) -> Vec<String> {
    let mut chain = vec![output_s.to_string()];

    let mut search = output_s;
    loop {
        let k = maps.keys().find(|k| k.1 == search);
        if let Some(k) = k {
            chain.push(k.0.to_string());
            search = k.0;
        } else {
            break;
        }
    }

    chain.remove(chain.len() - 1);
    chain.reverse();

    chain
}

fn get_map_src_dst(input: &str) -> (&str, &str) {
    let mut it = input.split("-to-");
    (
        it.next().unwrap(),
        it.next().unwrap().split(' ').next().unwrap(),
    )
}

fn get_dst_in_map(src: &u64, map: &[(Range<u64>, Range<u64>)]) -> u64 {
    for range in map {
        if range.0.contains(src) {
            let distance = src - range.0.start;
            let dst = range.1.start + distance;

            return dst;
        }
    }

    src.to_owned()
}

fn get_maps<'a, I>(input: &mut I) -> Vec<(Range<u64>, Range<u64>)>
where
    I: Iterator<Item = &'a str>,
{
    let mut ranges = Vec::new();

    loop {
        let line = input.next().unwrap();
        if line.is_empty() {
            break;
        }

        let maps = get_map(line);
        ranges.push(maps);
    }

    ranges
}

fn get_map(input: &str) -> (Range<u64>, Range<u64>) {
    let numbers = get_number_from_list(input);
    let size = *numbers.last().unwrap() as u64;
    let dst_start = *numbers.first().unwrap() as u64;
    let dst_end = dst_start as u64 + size as u64;

    let src_start = *numbers.get(1).unwrap() as u64;
    let src_end = src_start + size;
    let dst_range = dst_start..dst_end;
    let src_range = src_start..src_end;
    (src_range, dst_range)
}

fn get_seed_ranges(input: &str) -> Vec<Range<u64>> {
    let mut seeds = Vec::new();

    let mut it = input.split(':').filter(|l| !l.is_empty());
    it.next();

    let seed_str = it.next();
    let seed_str = seed_str.unwrap().split(' ').filter(|l| !l.is_empty());
    for s in seed_str {
        seeds.push(get_number(s) as u64);
    }

    assert!(seeds.len() % 2 == 0);

    let mut ranges = Vec::new();
    let mut i = 0;
    loop {
        let start = seeds.get(i).unwrap().to_owned();
        let end = start + seeds.get(i + 1).unwrap();

        i += 1;
        if i > seeds.len() / 2 {
            break;
        }

        ranges.push(start..end);
    }

    ranges
}

fn get_seeds(input: &str) -> Vec<u64> {
    let mut seeds = Vec::new();

    let mut it = input.split(':').filter(|l| !l.is_empty());
    it.next();

    let seed_str = it.next();
    let seed_str = seed_str.unwrap().split(' ').filter(|l| !l.is_empty());
    for s in seed_str {
        seeds.push(get_number(s) as u64);
    }

    seeds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = include_str!("../input.txt");
        let res = Part1::run(input);
        assert_eq!(res, 214922730);
    }

    #[test]
    fn part2() {
        let input = include_str!("../input.txt");
        let res = Part2::run(input);
        assert_eq!(res, 214922730);
    }

    #[test]
    fn example_part1() {
        let input = r"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        ";

        let res = Part1::run(input);
        assert_eq!(res, 35);
    }
}
