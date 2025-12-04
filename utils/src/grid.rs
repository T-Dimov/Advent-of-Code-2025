use std::fs;
use std::io::{BufRead, BufReader};

pub struct Grid<T>
{
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy)]
pub enum Neighbors
{
    Orthogonal,
    Diagonal,
    All,
}

const ORTHOGONAL: &[(i32, i32)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];
const DIAGONAL: &[(i32, i32)] = &[(-1, -1), (1, -1), (-1, 1), (1, 1)];
const ALL: &[(i32, i32)] = &[
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];

fn read_lines(path: &str) -> (Vec<String>, usize, usize)
{
    let lines: Vec<String> = BufReader::new(fs::File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let height = lines.len();
    let width = if height > 0 { lines[0].len() } else { 0 };

    (lines, width, height)
}

impl<T: Clone> Grid<T>
{
    pub fn from_file_with<F>(path: &str, parser: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let (lines, width, height) = read_lines(path);
        let data = lines
            .into_iter()
            .flat_map(|line| line.chars().map(&parser).collect::<Vec<_>>())
            .collect();
        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T
    {
        self.data[y * self.width + x].clone()
    }

    pub fn set(&mut self, x: usize, y: usize, val: T)
    {
        self.data[y * self.width + x] = val;
    }

    pub fn for_each_neighbor<F>(&self, x: usize, y: usize, mode: Neighbors, mut f: F)
    where
        F: FnMut(usize, usize, T),
    {
        let offsets = match mode
        {
            Neighbors::Orthogonal => ORTHOGONAL,
            Neighbors::Diagonal => DIAGONAL,
            Neighbors::All => ALL,
        };

        for (dx, dy) in offsets
        {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height
            {
                let (nx, ny) = (nx as usize, ny as usize);
                f(nx, ny, self.get(nx, ny));
            }
        }
    }
}

impl Grid<char>
{
    pub fn from_file(path: &str) -> Self
    {
        Self::from_file_with(path, |c| c)
    }
}

impl Grid<i64>
{
    pub fn from_file(path: &str) -> Self
    {
        Self::from_file_with(path, |c| c.to_digit(10).unwrap() as i64)
    }
}
