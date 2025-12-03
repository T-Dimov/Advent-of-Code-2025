use std::fs;
use std::io::{BufRead, BufReader};

pub fn run()
{
    println!("Day 03");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32
{
    BufReader::new(fs::File::open("input/day03/input.txt").unwrap())
        .lines()
        .map(|line| find_bank_joltage_2_batteries(line.unwrap().as_str()))
        .sum::<u32>()
}

fn part2() -> u64
{
    todo!()
}

fn find_max_joltage_in_bank(bank: &str) -> (usize, u32)
{
    bank.chars()
        .enumerate()
        .fold((0, 0), |(max_ind, max_jolt), (bat_ind, bat_jolt)| {
            let jolt = bat_jolt.to_digit(10).unwrap();
            if jolt > max_jolt
            {
                (bat_ind, jolt)
            }
            else
            {
                (max_ind, max_jolt)
            }
        })
}

fn find_bank_joltage_2_batteries(bank: &str) -> u32
{
    let len = bank.len();
    let len_1 = len - 1;
    assert!(len > 1);

    let (max_ind, max_jolt) = find_max_joltage_in_bank(bank);
    if max_ind == len_1
    {
        let (_, second_highest_joltage) = find_max_joltage_in_bank(&bank[..len_1]);
        second_highest_joltage * 10 + max_jolt
    }
    else
    {
        let (_, second_max_joltage) = find_max_joltage_in_bank(&bank[max_ind + 1..]);
        max_jolt * 10 + second_max_joltage
    }
}
