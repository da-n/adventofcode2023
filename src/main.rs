mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {
    // Day one.
    if let Err(err) = day_01::part_one() {
        eprintln!("Error: {}", err);
    }
    if let Err(err) = day_01::part_two() {
        eprintln!("Error: {}", err);
    }

    // Day two.
    match day_02::totals() {
        Ok(totals) => {
            println!("day 02: total - part one: {}", totals.0);
            println!("day 02: total - part two: {}", totals.1);
        }
        Err(err) => eprintln!("error: {}", err),
    }

    // Day three.
    match day_03::totals() {
        Ok(totals) => {
            println!("day 03: total - part one: {}", totals.0);
            println!("day 03: total - part two: {}", totals.1);
        }
        Err(err) => eprintln!("error: {}", err),
    }

    // Day four.
    match day_04::totals() {
        Ok(totals) => {
            println!("day 04: total - part one: {}", totals.0);
            println!("day 04: total - part two: {}", totals.1);
        }
        Err(err) => eprintln!("error: {}", err),
    }
}
