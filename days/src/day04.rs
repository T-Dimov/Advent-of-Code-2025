use utils::Grid;

pub fn run()
{
    println!("Day 04");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i64
{
    let grid = Grid::<char>::from_file("input/day04/input.txt");

    let mut accessible_rolls_count = 0;

    for x in 0..grid.width
    {
        for y in 0..grid.height
        {
            if is_roll_accessible(&grid, x, y)
            {
                accessible_rolls_count += 1;
            }
        }
    }

    accessible_rolls_count
}

fn part2() -> i64
{
    let mut grid = Grid::<char>::from_file("input/day04/input.txt");

    let mut removable_rolls_count = 0;

    loop
    {
        let mut removed_rolls_iteration = 0;

        for x in 0..grid.width
        {
            for y in 0..grid.height
            {
                if is_roll_accessible(&grid, x, y)
                {
                    removed_rolls_iteration += 1;
                    grid.set(x, y, 'x');
                }
            }
        }

        if removed_rolls_iteration == 0
        {
            break;
        }
        else
        {
            removable_rolls_count += removed_rolls_iteration;
        }
    }

    removable_rolls_count
}

fn is_roll_accessible(grid: &Grid<char>, x: usize, y: usize) -> bool
{
    if grid.get(x, y) == '@'
    {
        let mut neighbouring_roll_count = 0;

        grid.for_each_neighbor(x, y, utils::Neighbors::All, |_, _, neighbour| {
            if neighbour == '@'
            {
                neighbouring_roll_count += 1;
            }
        });

        neighbouring_roll_count <= 3
    }
    else
    {
        false
    }
}
