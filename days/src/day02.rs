use std::fs;
use fancy_regex_macro::regex;

pub fn run()
{
    println!("Day 02");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i64
{
    process_file("input/day02/input.txt", |id| {
        let id_str = id.to_string();
        let len = id_str.len();
        let halves = id_str.split_at(len / 2);
        len % 2 == 0 && halves.0 == halves.1
    })
}

fn part2() -> i64
{
    let invalid_id_regex = regex!(r"^(\d+)\1+$");
    process_file("input/day02/input.txt", |id| {
        invalid_id_regex.is_match(&id.to_string()).unwrap()
    })
}

fn process_file<P>(path: &str, is_invalid_id: P) -> i64
where
    P: Fn(&i64) -> bool,
{
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|pair| {
            let mut parts = pair.split('-');
            parts.next().unwrap().parse::<i64>().unwrap()
                ..=parts.next().unwrap().parse::<i64>().unwrap()
        })
        .map(|range| range.filter(&is_invalid_id).sum::<i64>())
        .sum::<i64>()
}
