use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

pub fn part_one() -> Result<(), Box<dyn Error>> {
    let file_path = format!(
        "{}/data/input_day_01.txt",
        env::current_dir().unwrap().display()
    );
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut total: i32 = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let line = line.as_str();
                match extract_digits(line) {
                    Ok(line_total) => total += line_total,
                    Err(err) => eprintln!("error reading file: {}", err),
                }
            }
            Err(err) => {
                eprintln!("error reading line from file: {}", err);
            }
        }
    }

    println!("day 01: total - part one: {}", total);
    Ok(())
}

pub fn part_two() -> Result<(), Box<dyn Error>> {
    let file_path = format!(
        "{}/data/input_day_01.txt",
        env::current_dir().unwrap().display()
    );
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut total: i32 = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let line = line.as_str();
                match extract_digits_and_words(line) {
                    Ok(line_total) => total += line_total,
                    Err(err) => eprintln!("error reading file: {}", err),
                }
            }
            Err(err) => {
                eprintln!("error reading line from file: {}", err);
            }
        }
    }

    println!("day 01: total - part two: {}", total);
    Ok(())
}

fn extract_digits(line: &str) -> Result<i32, ParseIntError> {
    let mut nums = Vec::new();
    for c in line.chars() {
        if let Some(digit) = c.to_digit(10) {
            nums.push(digit);
            continue;
        }
    }

    format!(
        "{}{}",
        nums.first().unwrap().to_string(),
        nums.last().unwrap().to_string()
    )
    .parse()
}

fn extract_digits_and_words(line: &str) -> Result<i32, ParseIntError> {
    let mapper: HashMap<&str, i32> = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let chars: Vec<char> = line.chars().collect();
    let mut nums = Vec::new();
    let mut partial = String::with_capacity(4);

    // Build up each char to a partial and try to match this to the mapper.
    for i in 0..line.len() {
        partial.clear();
        for ii in 0..(line.len() - i) {
            // Add a char to the partial until we get a match or fall through.
            partial.push(chars[i + ii]);
            if let Some(&num_word) = mapper.get(partial.as_str()) {
                nums.push(num_word);
                break;
            }
        }
    }

    Ok(format!(
        "{}{}",
        nums.first().unwrap().to_string(),
        nums.last().unwrap().to_string()
    )
    .parse()?)
}

#[cfg(test)]
mod tests {
    use super::{extract_digits, extract_digits_and_words};

    fn test_extract_total_int_case(input: &str, expected: i32) -> Result<(), String> {
        match extract_digits(input) {
            Ok(result) => {
                if result != expected {
                    Err(format!("want {}, got {}", expected, result))
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(format!("{}", err.to_string())),
        }
    }

    #[test]
    fn test_extract_total_int() -> Result<(), String> {
        [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ]
        .iter()
        .try_for_each(|(input, expected)| test_extract_total_int_case(*input, *expected))?;

        Ok(())
    }

    fn test_extract_total_mixed_case(input: &str, expected: i32) -> Result<(), String> {
        match extract_digits_and_words(input) {
            Ok(result) => {
                if result != expected {
                    Err(format!("want {}, got {}", expected, result))
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(format!("{}", err.to_string())),
        }
    }

    #[test]
    fn test_extract_total_mixed() -> Result<(), String> {
        [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            ("onetwoone", 11),
            ("eightwo", 82),
        ]
        .iter()
        .try_for_each(|(input, expected)| test_extract_total_mixed_case(*input, *expected))?;

        Ok(())
    }
}
