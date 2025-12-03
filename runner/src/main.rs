fn main()
{
    // Change this to run different days
    let day = 3;

    match day
    {
        1 => days::day01::run(),
        2 => days::day02::run(),
        3 => days::day03::run(),
        _ => println!("Day {} not implemented yet", day),
    }
}
