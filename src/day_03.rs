use num::ToPrimitive;
use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, Cursor};
use std::{env, fs};

#[derive(Clone, PartialEq, Debug)]
enum CellType {
    Period,
    Symbol,
    Digit,
    Gear,
}

#[derive(Clone, Debug)]
struct Cell {
    cell_value: usize,
    cell_type: CellType,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Cell {
    fn new(cell_value: usize, cell_type: CellType) -> Self {
        Cell {
            cell_value,
            cell_type,
        }
    }

    fn is_digit(&self) -> bool {
        self.cell_type == CellType::Digit
    }

    fn is_symbol_or_gear(&self) -> bool {
        self.cell_type == CellType::Symbol || self.cell_type == CellType::Gear
    }

    fn is_gear(&self) -> bool {
        self.cell_type == CellType::Gear
    }
}

pub fn totals() -> Result<(usize, usize), Box<dyn Error>> {
    let file_path = format!(
        "{}/data/input_day_03.txt",
        env::current_dir().unwrap().display()
    );
    Ok((
        sum_part_numbers(fs::read_to_string(&file_path)?)?,
        sum_gear_ratios(fs::read_to_string(&file_path)?)?,
    ))
}

fn sum_part_numbers(input: String) -> Result<usize, Box<dyn Error>> {
    let schem = create_padded_schematic(input)?;
    let mut sum: usize = 0;

    for (row_index, row) in schem.iter().enumerate() {
        // Skip first and last rows as these are just for padding.
        if row_index == 0 || row_index == (schem.len() - 1) {
            continue;
        }

        let mut digits: Vec<usize> = vec![];

        for (cell_index, cell) in row.iter().enumerate() {
            if cell.is_digit() {
                digits.push(cell.cell_value);
                // The last cell will always be padding so skip to next.
                continue;
            }

            // Check if there are any digits to handle, otherwise skip.
            if digits.is_empty() {
                continue;
            }

            let mut is_part: bool = false;
            let scan_width: usize = digits.len();

            // Scan around the cell.
            for i in 0..scan_width {
                // Scan above.
                if schem[row_index - 1][(cell_index - scan_width) + i].is_symbol_or_gear() {
                    is_part = true;
                }

                // Scan below.
                if schem[row_index + 1][(cell_index - scan_width) + i].is_symbol_or_gear() {
                    is_part = true;
                }

                // Scan left.
                if i == 0 {
                    if schem[row_index][cell_index - (scan_width + 1)].is_symbol_or_gear() {
                        is_part = true;
                    }

                    if schem[row_index - 1][cell_index - (scan_width + 1)].is_symbol_or_gear() {
                        is_part = true;
                    }

                    if schem[row_index + 1][cell_index - (scan_width + 1)].is_symbol_or_gear() {
                        is_part = true;
                    }
                }

                // Scan right.
                if i == (scan_width - 1) {
                    if schem[row_index][cell_index].is_symbol_or_gear() {
                        is_part = true;
                    }

                    if schem[row_index - 1][cell_index].is_symbol_or_gear() {
                        is_part = true;
                    }

                    if schem[row_index + 1][cell_index].is_symbol_or_gear() {
                        is_part = true;
                    }
                }
            }

            if is_part {
                // We now need to convert the vec of usize into strings, concatenate them, and
                // convert back to usize :facepalm: Must be a nicer way than this...
                let string_vals: Vec<String> = digits.iter().map(|&n| n.to_string()).collect();
                let concat_string: String = string_vals.join("");
                sum += concat_string.parse::<usize>()?;
            }

            digits.clear();
        }
    }

    Ok(sum)
}

fn sum_gear_ratios(input: String) -> Result<usize, Box<dyn Error>> {
    let schem = create_padded_schematic(input)?;
    let mut gear_ratios: Vec<usize> = vec![];

    for (row_index, row) in schem.iter().enumerate() {
        // Skip first and last rows as these are just for padding.
        if row_index == 0 || row_index == (schem.len() - 1) {
            continue;
        }

        for (cell_index, cell) in row.iter().enumerate() {
            // Skip first cell as this is just padding and anything that's not a gear.
            if cell_index == 0 {
                continue;
            } else if !cell.is_gear() {
                continue;
            }

            let mut coordinates = get_adjacent_coordinates(row_index, cell_index);

            if let Some(gear_ratio) = determine_gear_ratio(&schem, &mut coordinates) {
                gear_ratios.push(gear_ratio)
            }
        }
    }

    Ok(gear_ratios.iter().sum())
}

fn get_adjacent_coordinates(row: usize, col: usize) -> HashSet<Coordinate> {
    // Build up a vec of all the surrounding cell co-ordinates.
    let mut coordinates: HashSet<Coordinate> = HashSet::new();
    // Top left.
    coordinates.insert(Coordinate {
        x: row - 1,
        y: col - 1,
    });
    // Top middle.
    coordinates.insert(Coordinate {
        x: row - 1,
        y: (col - 1) + 1,
    });
    // Top right.
    coordinates.insert(Coordinate {
        x: row - 1,
        y: (col - 1) + 2,
    });
    // Mid right.
    coordinates.insert(Coordinate { x: row, y: col + 1 });
    // Bottom right.
    coordinates.insert(Coordinate {
        x: row + 1,
        y: (col - 1) + 2,
    });
    // Bottom middle.
    coordinates.insert(Coordinate {
        x: row + 1,
        y: (col - 1) + 1,
    });
    // Bottom left.
    coordinates.insert(Coordinate {
        x: row + 1,
        y: col - 1,
    });
    // Mid left.
    coordinates.insert(Coordinate { x: row, y: col - 1 });

    coordinates
}

fn determine_gear_ratio(
    schem: &[Vec<Cell>],
    coordinates: &mut HashSet<Coordinate>,
) -> Option<usize> {
    let mut part_nums: Vec<usize> = vec![];
    let mut scanned: HashSet<Coordinate> = HashSet::new();

    for coordinate in coordinates.iter() {
        if scanned.contains(coordinate) {
            continue;
        }

        if !schem[coordinate.x][coordinate.y].is_digit() {
            scanned.insert(coordinate.clone());
            continue;
        }

        // This is a digit, walk left until we get to the first digit of the entire part number.
        let mut part_num: String = String::new();
        let mut pos: usize = coordinate.y;
        loop {
            if !schem[coordinate.x][pos].is_digit() {
                break;
            }
            pos -= 1;
        }

        // We now have the starting column, walk right until we determine the entire part number.
        loop {
            part_num.push_str(&schem[coordinate.x][pos].cell_value.to_string());
            let curr_coordinate = Coordinate {
                x: coordinate.x,
                y: pos,
            };
            scanned.insert(curr_coordinate.clone());
            pos += 1;
            if !schem[coordinate.x][pos].is_digit() {
                break;
            }
        }

        if !part_num.is_empty() {
            part_nums.push(part_num.parse::<usize>().ok()?);
        }
    }

    // It is only considered a gear if it has exact 2 adjacent part numbers.
    let mut result: usize = 0;
    if part_nums.len() == 2 {
        // The gear ratio is the result of multiplying the part numbers together.
        result = part_nums.iter().fold(1, |acc, &num| acc * num);
    }
    Some(result)
}

fn create_padded_schematic(input: String) -> Result<Vec<Vec<Cell>>, Box<dyn Error>> {
    let cursor = Cursor::new(input);
    let reader = BufReader::new(cursor);

    let mut schematic: Vec<Vec<Cell>> = Vec::new();
    let mut line_len: usize = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_default();

        // Skip empty lines.
        if line.is_empty() {
            continue;
        }

        // We only need to set the length once.
        if i == 0 {
            line_len = line.len().to_usize().unwrap();
        }

        let mut row: Vec<Cell> = Vec::new();
        for char in line.chars() {
            let (cell_value, cell_type): (usize, CellType);
            match char {
                '.' => {
                    cell_value = 0;
                    cell_type = CellType::Period;
                }
                '0'..='9' => {
                    cell_value = char
                        .to_digit(10)
                        .ok_or("invalid digit")?
                        .to_usize()
                        .ok_or("invalid usize")?;
                    cell_type = CellType::Digit;
                }
                '*' => {
                    cell_value = 0;
                    cell_type = CellType::Gear;
                }
                _ => {
                    cell_value = 0;
                    cell_type = CellType::Symbol;
                }
            };

            let cell = Box::new(Cell::new(cell_value, cell_type));
            row.push(*cell);
        }

        // Pad the left and right to minimise out of bound checking.
        row.insert(0, Cell::new(0, CellType::Period));
        row.push(Cell::new(0, CellType::Period));

        schematic.push(row);
    }

    // Pad top and bottom to minimise out of bound checking.
    schematic.insert(
        0,
        (0..(line_len + 2).to_usize().unwrap_or_default())
            .map(|_| Cell::new(0, CellType::Period))
            .collect(),
    );
    schematic.push(
        (0..(line_len + 2).to_usize().unwrap_or_default())
            .map(|_| Cell::new(0, CellType::Period))
            .collect(),
    );

    Ok(schematic)
}

#[cfg(test)]
mod tests {
    use super::{sum_gear_ratios, sum_part_numbers};

    fn test_sum_part_numbers_case(input: &str, want: usize) -> Result<(), String> {
        match sum_part_numbers(String::from(input)) {
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
    fn test_sum_part_numbers() -> Result<(), String> {
        [
            (
                r#"*100......
.........."#,
                100,
            ),
            (
                r#"200*......
.........."#,
                200,
            ),
            (
                r#"300.......
...*......"#,
                300,
            ),
            (
                r#".400......
*........."#,
                400,
            ),
            (
                r#"...*......
500......."#,
                500,
            ),
            (
                r#"*.........
.600......"#,
                600,
            ),
            (
                r#"..*.......
.700......"#,
                700,
            ),
            (
                r#".800......
..*......."#,
                800,
            ),
            (
                r#".900......
...*......"#,
                900,
            ),
            (
                r#".10.......
.*........"#,
                10,
            ),
            (
                r#"..12......
.........*"#,
                0,
            ),
            (
                r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
                4361,
            ),
            (r#""#, 0),
            (r#"."#, 0),
            (r#".........."#, 0),
        ]
        .iter()
        .try_for_each(|(input, want)| test_sum_part_numbers_case(*input, *want))?;

        Ok(())
    }

    fn test_sum_gear_ratios_case(input: &str, want: usize) -> Result<(), String> {
        match sum_gear_ratios(String::from(input)) {
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
            (r#""#, 0),
            (r#"."#, 0),
            (r#".........."#, 0),
            (r#"1"#, 0),
            (r#".1"#, 0),
            (r#"1."#, 0),
            (r#".1."#, 0),
            (r#"1........"#, 0),
            (r#"........1"#, 0),
            (r#"....1...."#, 0),
            (r#".....100"#, 0),
            (
                r#"1.......
........"#,
                0,
            ),
            (
                r#"........
1.......
........"#,
                0,
            ),
            (
                r#"467..114..
...*......
..35..633."#,
                16345,
            ),
            (
                r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
                467835,
            ),
        ]
        .iter()
        .try_for_each(|(input, want)| test_sum_gear_ratios_case(*input, *want))?;

        Ok(())
    }
}
