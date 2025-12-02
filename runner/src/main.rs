fn main()
{
    // Change this to run different days
    let day = 2;

    match day
    {
        1 => days::day01::run(),
        2 => days::day02::run(),
        _ => println!("Day {} not implemented yet", day),
    }
}
