use std::fs;


fn get_color(set: &str, color: &str) -> u32 {
    for color_result in set.split(',') {
        let matched = color_result.find(color);
        match matched {
            Some(value) => {
                let (mut string_number, mut _color) = {
                    let mut parts = color_result.split_whitespace();
                    (
                        parts.next().unwrap_or(""),
                        parts.next().unwrap_or(""),
                    )
                };
                println!("Split -{}- into -{}- and -{}-", color_result, string_number, _color);

                let number: Result<i32, _> = string_number.parse();
                match number {
                    Ok(parsed) => {
                        return parsed as u32;
                    }
                    Err(e) => {
                        println!("Error parsing string: -{}-", string_number);
                    }
                }
            }
            None => {
                continue;
            }
        }
    }
    return 0;
}


fn is_possible_game(line: String, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    let mut possible_red = 0;
    let mut possible_green = 0;
    let mut possible_blue = 0;
    println!("Line: {line}");
    for set in line.split(';') {
        println!("Current \x1b[31mred: {}\x1b[0m, \x1b[32mgreen: {}\x1b[0m, \x1b[34mblue: {}\x1b[0m", possible_red, possible_green, possible_blue);
        println!("Checking set {}", set);
        let set_red = get_color(set, "red");
        if possible_red < set_red {
            possible_red = set_red;
        }
        let set_green = get_color(set, "green");
        if possible_green < set_green {
            possible_green = set_green;
        }
        let set_blue = get_color(set, "blue");
        if possible_blue < set_blue {
            possible_blue = set_blue;
        }
    }
    if possible_red > max_red {
        return false;
    }
    if possible_green > max_green {
        return false;
    }
    if possible_blue > max_blue {
        return false;
    }
    return true;
}


fn get_just_sets(line: &str) -> String {
    let matched = line.find(':');
    match matched {
        Some(value) => {
            return line[value + 1..].to_string();
        }
        None => {
            return line.to_string();
        }
    }
}


fn main() {
    let MAX_RED = 12;
    let MAX_GREEN = 13;
    let MAX_BLUE = 14;
    let file_contents = fs::read_to_string("input").expect("LogRocket: Should have been able to read the file");
    let mut total = 0;
    let mut game_id = 1;
    for line in file_contents.lines() {
        let game = get_just_sets(line);
        if is_possible_game(game, MAX_RED, MAX_GREEN, MAX_BLUE){
            total += game_id;
        }
        game_id += 1;
    }
    println!("Total of possible game ids: {}", total);
}
