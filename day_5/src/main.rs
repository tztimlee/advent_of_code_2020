use core::panic;
use std::{str::FromStr, fs};
fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    let boarding_passes = contents.lines().map(|pass| Identifier::from_str(pass).unwrap());

    let mut seats: Vec<usize> = boarding_passes
        .filter(|id| id.row != 1 && id.row != 127)
        .map(|x| x.get_seat_number())
        .collect();
    
    seats.sort();

    let mut prev = seats[0];
    seats.remove(0);

    for seat in seats{
        if prev + 1 != seat {
            println!("Prev: {}, Next: {}", prev, seat);
            break;
        } else {
            prev = seat;
        }
    }
    // let x = Identifier::from_str("BFFFBBFRRR").unwrap();
    
    // for i in boarding_passes{
    //     println!("{:?}", i);
    // }
    
}

#[derive(Debug)]
struct Identifier {
    row: usize,
    column: usize,
}

impl Identifier {
    fn get_seat_number(&self) -> usize {
        self.row * 8 + self.column
    }

    fn get_row_number(criteria: &str) -> usize{
        let mut min = 0;
        let mut max = 127;

        for n in criteria.chars(){

            let mid = ((max - min) / 2) + min;

            match n {
                'F' => max = mid,
                'B' => min = mid + 1,
                _ => panic!("WE GOT A WEIRD LETTER")
            }
        }

        //max and min should be the same by this point
        return min;
    }

    fn get_column_number(criteria: &str) -> usize{
        let mut min = 0;
        let mut max = 7;

        for n in criteria.chars(){

            let mid = ((max - min) / 2) + min;

            match n {
                'L' => max = mid,
                'R' => min = mid + 1,
                _ => panic!("WE GOT A WEIRD LETTER")
            }
        }

        //max and min should be the same by this point
        return min;
    }
}

impl FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = Identifier::get_row_number(&s[..7]);
        let column = Identifier::get_column_number(&s[7..]);

        return Ok(Identifier{row, column})
    }
}
