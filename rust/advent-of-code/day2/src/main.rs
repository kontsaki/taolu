use regex::{Captures, Regex};
use std::fs;
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

fn main() {
    let valid_passwords = get_passwords("input")
        .iter()
        .filter(|pass| is_valid_new(pass))
        .count();
    println!("Valid passwords: {}", valid_passwords);
}

fn _is_valid_old(password: &PasswordLine) -> bool {
    let count = password.password.matches(password.character).count();
    password.min <= count && count <= password.max
}

fn is_valid_new(password: &PasswordLine) -> bool {
    let first_position = password
        .password
        .chars()
        .nth(password.min - 1)
        .unwrap_or('!');
    let second_position = password
        .password
        .chars()
        .nth(password.max - 1)
        .unwrap_or('!');
    first_position == password.character && second_position != password.character
        || first_position != password.character && second_position == password.character
}

#[derive(Debug)]
struct PasswordLine {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl TryFrom<Captures<'_>> for PasswordLine {
    type Error = ();
    fn try_from(caps: Captures<'_>) -> Result<Self, ()> {
        Ok(PasswordLine {
            min: PasswordLine::parse(&caps, "min")?,
            max: PasswordLine::parse(&caps, "max")?,
            character: PasswordLine::parse(&caps, "character")?,
            password: PasswordLine::parse(&caps, "password")?,
        })
    }
}

impl PasswordLine {
    fn parse<T: FromStr>(caps: &Captures<'_>, name: &str) -> Result<T, ()> {
        Ok(caps
            .name(name)
            .ok_or(())?
            .as_str()
            .parse::<T>()
            .or(Err(()))?)
    }
}

fn get_passwords(filename: &str) -> Vec<PasswordLine> {
    let contents = fs::read_to_string(filename).expect("WHERE IS FILE");
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<character>\w): (?P<password>\w+)").unwrap();
    contents
        .lines()
        .filter_map(|line| re.captures(line))
        .filter_map(|cap| cap.try_into().ok())
        .collect()
}
