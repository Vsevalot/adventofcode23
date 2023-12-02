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


fn get_game_power(line: String) -> u32 {
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
    return possible_red * possible_blue * possible_green;
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
    let file_contents = fs::read_to_string("input").expect("LogRocket: Should have been able to read the file");
    let mut total = 0;
    for line in file_contents.lines() {
        let game = get_just_sets(line);
        total += get_game_power(game);
    }
    println!("Total of game powers is: {}", total);
}
