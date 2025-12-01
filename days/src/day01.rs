use std::fs;
use std::io::{BufRead, BufReader};

pub fn run()
{
    println!("Day 01");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i64
{
    let mut dial_pos = 50;
    let mut dial_0_nums = 0;

    let buff_reader = BufReader::new(fs::File::open("input/day01/input.txt").unwrap());

    for line in buff_reader.lines()
    {
        dial_pos += decode_dial_dir(line.unwrap().as_str()) + 100;
        dial_pos %= 100;
        if dial_pos == 0
        {
            dial_0_nums += 1;
        }
    }

    dial_0_nums
}

fn part2() -> i64
{
    let mut dial_pos = 50;
    let mut dial_pass_trough_0 = 0;

    let buff_reader = BufReader::new(fs::File::open("input/day01/input.txt").unwrap());

    for line in buff_reader.lines()
    {
        let turn = decode_dial_dir(line.unwrap().as_str());

        dial_pos += turn;

        if turn > 0
        {
            dial_pass_trough_0 += dial_pos / 100;

            dial_pos %= 100;
        }
        else if dial_pos == 0
        {
            dial_pass_trough_0 += 1;
        }
        else if dial_pos < 0
        {
            dial_pass_trough_0 += turn.abs() / 100 + (if dial_pos == turn { 0 } else { 1 });

            dial_pos %= 100;
            dial_pos += 100;
        }
    }

    dial_pass_trough_0
}

fn decode_dial_dir(instruction: &str) -> i64
{
    match instruction.chars().next().unwrap()
    {
        'L' => -(instruction[1..].parse::<i64>().unwrap()),
        'R' => instruction[1..].parse::<i64>().unwrap(),
        _ => panic!("Unexpected first character of instruction!"),
    }
}
