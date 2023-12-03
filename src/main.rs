fn main() {
    let my_str = include_str!("puzzle_input.txt");

    let x: Vec<String> = my_str.lines().map(String::from).collect();

    let total: u32 = x.iter().map(|a| get_calibration_value(a)).flatten().sum();
    println!("{:?}", total)
}

fn get_calibration_value(line: &str) -> Option<u32> {
    let y = line.chars().filter(|x| x.is_numeric());

    let mut z = y.clone();
    let m = y.clone();

    let fist_char = z.nth(0)?;
    let last_char = m.last()?;

    let s1 = fist_char.to_string();
    let mut b = [0; 2];
    let s2 = last_char.encode_utf8(&mut b);

    let c = s1 + s2;
    match c.parse::<u32>() {
        Result::Ok(v) => Option::Some(v),
        Result::Err(_) => Option::None,
    }
}
