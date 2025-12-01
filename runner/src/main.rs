fn main()
{
    // Change this to run different days
    let day = 1;

    match day
    {
        1 => days::day01::run(),
        _ => println!("Day {} not implemented yet", day),
    }
}
