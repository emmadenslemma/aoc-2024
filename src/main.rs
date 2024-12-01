mod day_01;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", day_01::part1()?);
    println!("{}", day_01::part2()?);

    Ok(())
}
