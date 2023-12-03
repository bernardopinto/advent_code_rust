use std::{fmt::format, string::ParseError};


fn main() {
    let my_str = include_str!("puzzle_input.txt");

    let x: Vec<String> = my_str.lines().map(String::from).collect();

    let total: u32 = x.iter().map(|a| get_calibration_value(a)).flatten().sum();
    println!("{:?}", total)

}

fn get_calibration_value(line: &str) -> Option<u32> {
    let y = 
    line.chars()
    .filter(|x| x.is_numeric());
    // .map(|x| x.to_digit(10).unwrap());


    let mut z = y.clone();
    let m = y.clone();

    let first_char_maybe = z.nth(0);
    let last_char_maybe = m.last();

    let fist_char = first_char_maybe?;
    let last_char = last_char_maybe?;

    let s1 = fist_char.to_string();
    let mut b = [0; 2];
    let s2 = last_char.encode_utf8(&mut b);

    let c = s1 + s2;
    match c.parse::<u32>() {
        Result::Ok(v) => Option::Some(v),
        Result::Err(_) => Option::None
    }
}

