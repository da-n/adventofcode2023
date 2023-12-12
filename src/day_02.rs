use regex::{Captures, Regex};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
const EXPRESSION: &str =
    r"(Game\s(?P<game>\d+))|(?P<cubes>(?P<quantity>\d+)\s(?P<colour>\w+))|(?P<sep>;)|(?P<nl>\n)|$";
const GAME: &str = "game";
const CUBES: &str = "cubes";
const QUANTITY: &str = "quantity";
const COLOUR: &str = "colour";
const SEP: &str = "sep";
const NL: &str = "nl";
const BLUE: &str = "blue";
const RED: &str = "red";
const GREEN: &str = "green";

pub fn totals() -> Result<(i32, i32), Box<dyn Error>> {
    let file_path = format!(
        "{}/data/input_day_02.txt",
        env::current_dir().unwrap().display()
    );
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Hack to ensure there is an empty line at the bottom of the file, this is to avoid doing the
    // expensive part of parsing regex on every line and instead do it once on the entire file.
    contents.push('\n');

    let re = Regex::new(EXPRESSION)?;
    let mut caps: Vec<Captures> = vec![];
    caps.extend(re.captures_iter(&contents));

    Ok((line_values_total(&caps)?, line_powers_total(&caps)?))
}

fn line_values_total(caps: &Vec<Captures>) -> Result<i32, Box<dyn Error>> {
    const MAX_BLUES: i32 = 14;
    const MAX_REDS: i32 = 12;
    const MAX_GREENS: i32 = 13;
    let (mut total, mut game, mut blues, mut reds, mut greens): (i32, i32, i32, i32, i32) =
        (0, 0, 0, 0, 0);

    for cap in caps.into_iter() {
        if let Some(value) = cap.name(GAME) {
            game = value.as_str().parse::<i32>()?;
        } else if let Some(_) = cap.name(CUBES) {
            let mut quantity: i32 = 0;
            if let Some(quantity_match) = cap.name(QUANTITY) {
                quantity = quantity_match.as_str().parse::<i32>()?;
            }

            if let Some(colour) = cap.name(COLOUR) {
                match colour.as_str() {
                    BLUE => blues += quantity,
                    RED => reds += quantity,
                    GREEN => greens += quantity,
                    _ => {}
                }
            }
            if blues > MAX_BLUES || reds > MAX_REDS || greens > MAX_GREENS {
                // This game will score zero as one of the maximum cube quantities has been exceeded.
                game = 0;
            }
        } else if let Some(_) = cap.name(SEP) {
            // We are in a new group, reset quantities.
            (blues, reds, greens) = (0, 0, 0);
        } else if let Some(_) = cap.name(NL) {
            total += game;

            // We are in a new line so reset game score and quantities.
            (game, blues, reds, greens) = (0, 0, 0, 0);
        }
    }

    Ok(total)
}

fn line_powers_total(caps: &Vec<Captures>) -> Result<i32, Box<dyn Error>> {
    let (mut total, mut max_blue, mut max_red, mut max_green): (i32, i32, i32, i32) = (0, 0, 0, 0);

    for cap in caps.into_iter() {
        if let Some(_) = cap.name(CUBES) {
            let mut quantity: i32 = 0;
            if let Some(quantity_match) = cap.name(QUANTITY) {
                quantity = quantity_match.as_str().parse::<i32>()?;
            }

            if let Some(colour) = cap.name(COLOUR) {
                match colour.as_str() {
                    BLUE => {
                        if quantity > max_blue {
                            max_blue = quantity;
                        }
                    }
                    RED => {
                        if quantity > max_red {
                            max_red = quantity;
                        }
                    }
                    GREEN => {
                        if quantity > max_green {
                            max_green = quantity;
                        }
                    }
                    _ => {}
                }
            }
        } else if let Some(_) = cap.name(NL) {
            total += max_blue * max_red * max_green;

            // We are in a new line, reset quantities.
            (max_blue, max_red, max_green) = (0, 0, 0);
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::{line_powers_total, line_values_total, EXPRESSION};
    use regex::{Captures, Regex};

    #[test]
    fn test_line_values_total() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
        let mut matches: Vec<Captures> = vec![];
        matches.extend(Regex::new(EXPRESSION).unwrap().captures_iter(&input));

        let got = line_values_total(&matches).unwrap();

        assert_eq!(8, got);
    }

    #[test]
    fn test_line_powers_total() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
        let mut matches: Vec<Captures> = vec![];
        matches.extend(Regex::new(EXPRESSION).unwrap().captures_iter(&input));

        let got = line_powers_total(&matches).unwrap();

        assert_eq!(2286, got);
    }
}
