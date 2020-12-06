use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const NUM_ROWS: u8 = 128;
const NUM_COLS: u8 = 8;

#[derive(Debug)]
struct Seat {
    pub row: u8,
    pub col: u8,
    pub id: u32,
}

impl Seat {
    /// Create a new seat from a 10-character string.
    ///
    /// The first 7 characters must be either "F" or "B".
    ///
    /// The last 3 characters must be either "R" or "L".
    fn new(seat_string: &str) -> Result<Self> {
        if seat_string.len() != 10 {
            return Err(anyhow!(
                "String {} is the wrong length. Expected length of 10, found {}",
                seat_string,
                seat_string.len()
            ));
        }

        let row_chars = seat_string[0..7].chars();
        let col_chars = seat_string[7..10].chars();

        let mut row = 0..=(NUM_ROWS - 1);
        let mut col = 0..=(NUM_COLS - 1);

        for (i, row_char) in row_chars.enumerate() {
            let to_shrink_range_by = NUM_ROWS / (2u8.pow(i as u32 + 1));

            match row_char {
                'F' => {
                    row = (*row.start())..=(row.end() - to_shrink_range_by);
                }

                'B' => {
                    row = (row.start() + to_shrink_range_by)..=(*row.end());
                }

                _ => {
                    return Err(anyhow!(
                        "Expected character 'F' or 'B' in string {} at position {}, found {}",
                        seat_string,
                        i + 1,
                        row_char,
                    ))
                }
            }
        }

        assert!(
            row.start() == row.end(),
            "row location start and end are different! row.start() == {}, row.end() == {}",
            row.start(),
            row.end()
        );

        for (i, col_char) in col_chars.enumerate() {
            let to_shrink_range_by = NUM_COLS / (2u8.pow(i as u32 + 1));

            match col_char {
                'L' => {
                    col = (*col.start())..=(col.end() - to_shrink_range_by);
                }

                'R' => {
                    col = (col.start() + to_shrink_range_by)..=(*col.end());
                }

                _ => {
                    return Err(anyhow!(
                        "Expected character 'F' or 'B' in string {} at position {}, found {}",
                        seat_string,
                        i + 8,
                        col_char,
                    ))
                }
            }
        }

        assert!(
            col.start() == col.end(),
            "col location start and end are different! col.start() == {}, col.end() == {}",
            col.start(),
            col.end()
        );

        Ok(Self {
            row: *row.start(),
            col: *col.start(),
            id: (*row.start() as u32 * 8) + *col.start() as u32,
        })
    }
}

fn main() -> Result<()> {
    let input_file = File::open("./input/dec-05-part-01/input.txt")?;
    let reader = BufReader::new(input_file);

    let mut seats = Vec::new();

    for line in reader.lines() {
        seats.push(Seat::new(&line?)?);
    }

    seats.sort_unstable_by(|a, b| a.id.cmp(&b.id));

    let mut prev_seat_id = 0;
    let mut your_seat_id = None;

    for seat in seats {
        if prev_seat_id == 0 {
            // First iteration. Just set previous seat id and continue.
            prev_seat_id = seat.id;
            continue;
        }

        if prev_seat_id != seat.id - 1 {
            your_seat_id = Some(seat.id - 1);
        }

        prev_seat_id = seat.id;
    }

    if let Some(id) = your_seat_id {
        println!("Your seat's id is {}", id);
        Ok(())
    } else {
        Err(anyhow!("Could not locate your seat!"))
    }
}
