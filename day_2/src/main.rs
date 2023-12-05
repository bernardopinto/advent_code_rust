use regex::Regex;
fn main() {

    // enum Color {
    //     Red,
    //     Green,
    //     Blue,
    // }

    // struct CubeCount {
    //     color: Color,
    //     count: u32,
    // }

    // struct Game {
    //     id: u32,
    //     red_cubes: u32,
    //     green_cubes: u32,
    //     blue_cubes: u32,
    // }
    

    fn calculate_game(game_str: &str) -> Option<i32> {
        let re = Regex::new(r"Game (?<idx>\d+):(?<rest>.+?)$").unwrap();

        let game_regex = Regex::new(r"^ (?<n>\d+) (?<color>[a-z]+)$").unwrap();

        let capture_game = re.captures(game_str).unwrap();

        let idx = capture_game.name("idx").unwrap().as_str();
        let rest = capture_game.name("rest").unwrap().as_str();

        let included = String::from(rest).split(";").into_iter().map(|games| {
            println!("Printing games: {}", games);
            let list_bool = games.split(",").into_iter().map(|game| {
                println!("{}", game);
                let capture = game_regex.captures(game).unwrap();
                let count = capture.name("n").unwrap().as_str().parse::<i32>().unwrap();
                let color = capture.name("color").unwrap().as_str();
            
                match color {
                        "red" if count <= 12 => true,
                        "green" if count <= 13 => true,
                        "blue" if count <= 14 => true,
                        _ => false,
                }
            });
            list_bool.reduce(|acc, e| acc && e).unwrap()

        }).reduce(|acc, b| acc && b)?;


    if included {
            match idx.parse::<i32>() {
                Result::Ok(v) => return Option::Some(v),
                Result::Err(err) => {
                    println!("Failed to parse index string {}. Error {}", idx, err);
                    return Option::None;
                },
        }
 
    } else {return Option::None};

}

let my_str = include_str!("input.txt");
let x = my_str.lines().map(|g| calculate_game(g)).flatten().sum::<i32>();



println!("{}", x);
}
