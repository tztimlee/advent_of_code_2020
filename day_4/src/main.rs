use std::{fs, str::FromStr};
fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    let entries = contents.split("\n\n")
        .filter(|x| Passport::can_parse(x))
        .count();

    // println!("{}", entries);
    // for i in contents.chars(){
    //     print!("{}", i);
    //     // println!("{}", Passport::can_parse(i));
    // }
    println!("{}", entries);

    // let i = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    // hcl:#623a2f";

    // println!("{}", Passport::can_parse(i));
}
#[derive(Debug)]
struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: u16,
    hcl: String,
    ecl: String,
    pid: u32,
    cid: Option<u32>,
}

impl Passport {
    fn can_parse(s: &str) -> bool {
        match Self::from_str(s) {
            Ok(p) => {
                // println!("YAY, {:?}", p);
                return true;
            }
            Err(e) => {
                // println!("{}",e);
                return false;
            }
        }
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split_whitespace();

        let mut byr: u16 = 0;
        let mut iyr: u16 = 0;
        let mut eyr: u16 = 0;
        let mut hgt: u16 = 0;
        let mut hcl: String = String::new();
        let mut ecl: String = String::new();
        let mut pid: u32 = 0;
        let mut cid: Option<u32> = None;

        let mut count = 0;

        for i in data {
            count += 1;
            let info = &i[4..];
            match &i[0..3] {
                "byr" => byr = validate_years(info, 1920, 2002)?,
                "iyr" => iyr = validate_years(info, 2010, 2020)?,
                "eyr" => eyr = validate_years(info, 2020, 2030)?,
                "hgt" => hgt = validate_height(i)?,
                "hcl" => hcl = validate_hair_color(info)?,
                "ecl" => {
                    ecl = match info {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => info.to_string(),
                        _ => return Err("Invalid Hair Color".to_string()),
                    }
                }
                "pid" => {
                    if info.len() == 9 && info.chars().fold(true, |a, b| b.is_numeric() && a) {
                        pid = info.parse().unwrap();
                    } else {
                        return Err("Invalid pid".to_string());
                    }
                }
                "cid" => cid = Some(info.parse().unwrap()),
                _ => return Err("Unrecognised Record".to_string()),
            }
        }

        if count == 8 || (count == 7 && cid == None) {
            return Ok(Passport {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
                cid,
            });
        } else {
            // println!("{}", count);
            return Err("Incomplete Passport".to_string());
        }
    }
}

fn validate_years(info: &str, min: u16, max: u16) -> Result<u16, String> {
    let year: u16 = info.parse().unwrap();
    if info.len() == 4 && year >= min && year <= max {
        return Ok(year);
    } else {
        return Err("Invalid Birth Year".to_string());
    }
}

fn validate_height(i: &str) -> Result<u16, String> {
    if &i[i.len() - 2..] == "cm" {
        let height: u16 = i[4..i.len() - 2].parse().unwrap();
        if height >= 150 && height <= 193 {
            return Ok(height);
        }
    } else if &i[i.len() - 2..] == "in" {
        let height: u16 = i[4..i.len() - 2].parse().unwrap();
        if height >= 59 && height <= 76 {
            return Ok(height);
        }
    }
    return Err("Invalid Height".to_string());
}

fn validate_hair_color(info: &str) -> Result<String, String> {
    let mut char_iter = info.chars();
    if char_iter.next().unwrap() == '#' {
        let mut len_count = 0;
        for i in char_iter {
            // println!("{}",i);
            if (i.is_numeric()) || (i >= 'a' && i <= 'f') {
                len_count += 1;
            } else {
                return Err("Invalid Hair Color: invalid char".to_string());
            }
        }

        if len_count != 6 {
            return Err("Invalid Hair Color: incorrect len".to_string());
        } else {
            return Ok(info.to_string());
        }
    } else {
        return Err("Invalid Hair Color: Missing #".to_string());
    }
}
