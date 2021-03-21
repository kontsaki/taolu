fn main() {
    println!("Hello, codewars!");
}

/// [Build a square](https://www.codewars.com/kata/59a96d71dbe3b06c0200009c)
use std::iter::repeat;

fn generate_shape(n: i32) -> String {
    let row = repeat("+").take(n as usize).collect::<String>();
    repeat(row).take(n as usize).collect::<Vec<_>>().join("\n")
}

/// [Regex validation PIN code](https://www.codewars.com/kata/55f8a9c06c018a0d6e000132)
fn validate_pin(pin: &str) -> bool {
    println!("{}", pin);
    pin.chars().all(|c| c.is_numeric() && c.is_digit(10))
        && match pin.len() {
            4 | 6 => true,
            _ => false,
        }
}

/// [Dubstep](https://www.codewars.com/kata/551dc350bf4e526099000ae5)
fn song_decoder(song: &str) -> String {
    song.split("WUB")
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// [Descending Order](https://www.codewars.com/kata/5467e4d82edf8bbf40000155)
use itertools::Itertools;

fn descending_order(x: u64) -> u64 {
    x.to_string()
        .chars()
        .sorted()
        .rev()
        .join("")
        .parse()
        .unwrap_or(0)
}

/// [Ones and Zeros](https://www.codewars.com/kata/578553c3a1b8d5c40300037c)
use itertools::Itertools;

fn descending_order(x: u64) -> u64 {
    x.to_string()
        .chars()
        .sorted()
        .rev()
        .join("")
        .parse()
        .unwrap_or(0)
}

/// [Maximum Length Difference](https://www.codewars.com/kata/5663f5305102699bad000056)
use itertools::Itertools;

fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.is_empty() || a2.is_empty() {
        return -1;
    }
    let a1_lengths: Vec<_> = a1.iter().map(|word| word.len() as i32).sorted().collect();
    let a2_lengths: Vec<_> = a2.iter().map(|word| word.len() as i32).sorted().collect();
    let a1_minus_a2 = (a1_lengths[0] - a2_lengths.last().unwrap_or(&0)).abs();
    let a2_minus_a1 = (a2_lengths[0] - a1_lengths.last().unwrap_or(&0)).abs();
    if a1_minus_a2 > a2_minus_a1 {
        a1_minus_a2
    } else {
        a2_minus_a1
    }
}

/// [Your order, please](https://www.codewars.com/kata/55c45be3b2079eccff00010f)
fn order(sentence: &str) -> String {
    let mut words = sentence.split(' ').collect::<Vec<&str>>();
    words.sort_by(|a, b| find_numeric_char(a).cmp(&find_numeric_char(b)));
    words.join(" ")
}

fn find_numeric_char(word: &str) -> u32 {
    word.chars()
        .filter(|c| c.is_numeric())
        .nth(0)
        .unwrap_or('0')
        .to_digit(10)
        .unwrap_or(0)
}

/// [Deodorant Evaporator](https://www.codewars.com/kata/5506b230a11c0aeab3000c1f)
use std::iter::successors;

fn evaporator(_content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    successors(Some(1.0), |x| Some(x * (1.0 - evap_per_day as f64 / 100.0)))
        .take_while(|x| *x > threshold as f64 / 100.0)
        .count() as i32
}

/// [Speed Control](https://www.codewars.com/kata/56484848ba95170a8000004d)
fn gps(s: i32, x: Vec<f64>) -> i32 {
    x.iter()
        .zip(x.iter().skip(1))
        .map(|(x, y)| 3600 as f64 * (y - x) / s as f64)
        .filter_map(|x| Some(x.floor() as i32))
        .max()
        .unwrap_or(0)
}

/// [Growth of a Population](https://www.codewars.com/kata/563b662a59afc2b5120000c6)
use std::iter::successors;

fn nb_year_imperative(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut years = 0;
    let mut pn = p0;
    while pn < p {
        pn = (pn as f64 * (1.0 + percent / 100.0)) as i32 + aug;
        years += 1;
    }
    years
}

fn nb_year_declarative(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    successors(Some(p0), |&pn| Some(increase_for_a_year(pn, percent, aug)))
        .take_while(|pn| pn < &p)
        .count() as i32
}

fn increase_for_a_year(p: i32, percent: f64, aug: i32) -> i32 {
    (p as f64 * (1.0 + percent / 100.0)) as i32 + aug
}

/// [Array.diff](https://www.codewars.com/kata/523f5d21c841566fde000009)
fn array_diff_imperative<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    for item in a {
        if !b.contains(&item) {
            result.push(item);
        }
    }
    result
}
fn array_diff_declarative<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    return a
        .into_iter()
        .filter(|x| !b.contains(&x))
        .collect::<Vec<T>>();
}

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    array_diff_declarative(a, b)
}

/// [Give me a Diamond](https://www.codewars.com/kata/5503013e34137eeeaa001648)
fn print(n: i32) -> Option<String> {
    if n <= 0 || n % 2 == 0 {
        return None;
    } else {
        let mut result = String::new();
        for i in (1..=n).chain((1..n).rev()).filter(|i| i % 2 != 0) {
            result += " ".repeat(((n - i) / 2) as usize).as_str();
            result += "*".repeat(i as usize).as_str();
            result.push_str("\n");
        }
        Some(result)
    }
}

/// [String ends with?](https://www.codewars.com/kata/51f2d1cafc9c0f745c00037d)
fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

/// [Decode the Morse code ](https://www.codewars.com/kata/54b724efac3d5402db00065e)
const WORD_SEPARATOR: &str = "   ";

impl MorseDecoder {
    fn decode_morse(&self, encoded: &str) -> String {
        let mut decoded = String::new();
        for encoded_word in encoded.split(WORD_SEPARATOR) {
            for encoded_char in encoded_word.split_whitespace() {
                if let Some(val) = self.morse_code.get(encoded_char) {
                    decoded += val;
                }
            }
            decoded.push(' ');
        }
        decoded.trim().to_string()
    }
}
