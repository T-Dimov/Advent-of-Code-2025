fn main()
{
    // Change this to run different days
    let day = 4;

    match day
    {
        1 => days::day01::run(),
        2 => days::day02::run(),
        3 => days::day03::run(),
        4 => days::day04::run(),
        _ => println!("Day {} not implemented yet", day),
    }
}
