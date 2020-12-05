use std::{fs, str::FromStr};

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    
    let contents = contents.lines().map(|x| Password::from_str(x).unwrap());
    
    let mut valid = 0;
    for Password { min, max, char_val, password} in contents{
        // let occurances = password.chars().fold(0, |a, b| if b == char_val {a+1} else {a});

        // if min <= occurances && occurances <= max {
        //     valid += 1;
        // }

        let occurances: Vec<char> = password.chars().enumerate().filter(|i| i.0 == min-1 || i.0 == max-1 ).map(|i| i.1).collect();
        
        if (occurances[0] == char_val && occurances[1] != char_val) || (occurances[0] != char_val && occurances[1] == char_val){
            valid += 1;
        }
       
    }

    println!("Valid Passwords: {}", valid);
}

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    char_val: char,
    password: String
}

impl FromStr for Password {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(raw_password: &str) -> Result<Self, Self::Err> {
    
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let mut list = raw_password.split_whitespace();

        let mut bounds_list = list.next().unwrap().split('-');

        let min: usize = bounds_list.next().unwrap().parse::<usize>().unwrap();
        let max: usize = bounds_list.next().unwrap().parse::<usize>().unwrap();

        let char_val = list.next().unwrap().chars().nth(0).unwrap();
        let password: String = list.next().unwrap().to_string();

        Ok(Password { min, max, char_val, password })
    }
}