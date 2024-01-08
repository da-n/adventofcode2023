use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

struct ScratchCard {
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
}

pub fn totals() -> Result<(usize, usize), Box<dyn Error>> {
    let file_path = format!(
        "{}/data/input_day_04.txt",
        env::current_dir().unwrap().display()
    );
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    let mut scratch_cards: Vec<ScratchCard> = vec![];

    file.read_to_string(&mut contents)?;
    string_to_captures(&contents, &mut scratch_cards)?;

    Ok((sum_scratchcards(&scratch_cards)?, 0))
}

fn string_to_captures(contents: &str, cards: &mut Vec<ScratchCard>) -> Result<(), Box<dyn Error>> {
    for line in contents.lines() {
        let mut winning_numbers: Vec<usize> = vec![];
        let mut card_numbers: Vec<usize> = vec![];

        // First split on the colon and pipe.
        let parts: Vec<&str> = line.split(&[':', '|']).collect();

        // Get the digits from each segment and populate the cards.
        for (part_index, part) in parts.iter().enumerate() {
            part.trim()
                .split_whitespace()
                .for_each(|n| match n.parse::<usize>() {
                    Ok(parsed_num) => match part_index {
                        1 => winning_numbers.push(parsed_num),
                        2 => card_numbers.push(parsed_num),
                        _ => (),
                    },
                    Err(_) => {
                        return ();
                    }
                });
        }

        cards.push(ScratchCard {
            winning_numbers,
            card_numbers,
        });
    }

    Ok(())
}

fn sum_scratchcards(cards: &Vec<ScratchCard>) -> Result<usize, Box<dyn Error>> {
    let mut total_points: Vec<usize> = vec![];
    for card in cards {
        let num: HashSet<_> = card.card_numbers.clone().into_iter().collect();
        let win: HashSet<_> = card.winning_numbers.clone().into_iter().collect();
        let intersection: HashSet<_> = num.intersection(&win).cloned().collect();
        let mut points: usize = 0;
        for _ in intersection.iter() {
            if points == 0 {
                // Award first point.
                points = 1;
                continue;
            }
            points = points * 2;
        }
        total_points.push(points);
    }

    Ok(total_points.iter().sum())
}

#[cfg(test)]
mod tests {
    use crate::day_04::{string_to_captures, sum_scratchcards, ScratchCard};

    fn test_sum_gear_ratios_case(input: &str, want: usize) -> Result<(), String> {
        let contents = String::from(input);
        let mut scratch_cards: Vec<ScratchCard> = vec![];

        string_to_captures(&contents, &mut scratch_cards).unwrap_or_default();

        match sum_scratchcards(&scratch_cards) {
            Ok(got) => {
                if got != want {
                    Err(format!("want {}, got {}", want, got))
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(format!("{}", err.to_string())),
        }
    }

    #[test]
    fn test_sum_gear_ratios() -> Result<(), String> {
        [
            (r#"Card   1: 1 2 3 4 5 6 7 8 9 10 | 99 33 86 53 15 82 50 85 61 18 98 72 43 63 45 78 87 69 11 34 73 88 65 27 19"#, 0),
            (r#"Card   1: 1 2 3 4 5 6 7 8 9 10 | 1 33 86 53 15 82 50 85 61 18 98 72 43 63 45 78 87 69 11 34 73 88 65 27 19"#, 1),
            (r#"Card   1: 1 2 3 4 5 6 7 8 9 10 | 1 2 86 53 15 82 50 85 61 18 98 72 43 63 45 78 87 69 11 34 73 88 65 27 19"#, 2),
            (r#"Card   1: 1 2 3 4 5 6 7 8 9 10 | 1 2 3 4 5 82 50 85 61 18 98 72 43 63 45 78 87 69 11 34 73 88 65 27 19"#, 16),
            (r#"Card   1: 69 72 87 33 61 15  8 78 43 50 | 96 33 86 53 15 82 50 85 61  8 98 72 43 63 45 78 87 69 10 34 73 88 65 27 19"#, 512),
        ]
        .iter()
        .try_for_each(|(input, want)| test_sum_gear_ratios_case(*input, *want))?;

        Ok(())
    }
}
