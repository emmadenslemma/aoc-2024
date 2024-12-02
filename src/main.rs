mod days;

use days::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", day_02::part1()?);
    println!("{}", day_02::part2()?);

    Ok(())
}
