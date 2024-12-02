mod days;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        if let Some(arg) = args
            .iter()
            .map(|arg| arg.split('=').collect::<Vec<&str>>())
            .find(|arg| arg[0] == "-d" || arg[0] == "--day")
        {
            if let Ok(day) = arg[1].trim().parse() {
                print_day(day);

                return Ok(());
            }
        }
    }

    println!("        === ADVENT OF CODE ===");
    println!("PLEASE ENTER THE DAY YOU WISH TO VIEW:");

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input)?;

        if let Ok(day) = input.trim().parse() {
            print_day(day);

            return Ok(());
        }
    }
}

fn print_day(day: i32) {
    let (part1, part2) = days::run_solution(day);

    println!("  DAY {}", day);
    println!("PART 1: {}", part1);

    if part2 != 0 {
        println!("PART 2: {}", part2);
    }
}
