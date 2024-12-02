mod days;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("        === ADVENT OF CODE ===");
    println!("PLEASE ENTER THE DAY YOU WISH TO VIEW:");

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input)?;

        if let Ok(day) = input.trim().parse() {
            let (part1, part2) = days::run_solution(day);

            println!("  DAY {}", day);
            println!("PART 1: {}", part1);

            if part2 != 0 {
                println!("PART 2: {}", part2);
            }

            return Ok(());
        }
    }
}
