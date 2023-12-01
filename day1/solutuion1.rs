use std::env;
use std::fs;


fn get_last_idx(line: &str, to_find: &str) -> usize {
    let reversed_line: String = line.chars().rev().collect();
    let reversed_to_find: String = to_find.chars().rev().collect();
    let matched = reversed_line.find(&reversed_to_find);
    match matched {
        Some(value) => {
            let real_start = value as usize + to_find.len();
            // println!("{} is at {} of {} in {}", reversed_to_find, real_start, line.len(), reversed_line);
            return line.len() - real_start
        }
        None => {
            // We should not even get here - we call this function only when found a digit
            return line.len() - 1;
        }
    }
}


fn get_line_number(line: &str, debug: bool) -> u32 {
    let digits: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];  // I need a better name for this array :)
    let mut first_idx = line.len();
    let mut first = 0;
    let mut last_idx = 0;
    let mut last = 0;
    let mut first_round = true;
    for (index, &digit) in digits.iter().enumerate() {
        let matched = line.find(digit);
        match matched {
            Some(value) => {
                let position: usize = value as usize;
                if first_idx > position {
                    first_idx = position;
                    first = if index < 10 { index } else { index - 10 };
                    if first_round {
                        last_idx = get_last_idx(line, digit);
                        last = if index < 10 { index } else { index - 10 };
                        first_round = false;
                    }
                }
                println!("get last returned: {}", get_last_idx(line, digit));
                if last_idx < get_last_idx(line, digit){
                    last_idx = get_last_idx(line, digit);
                    last = if index < 10 { index } else { index - 10 };
                }

                if debug {
                    let mut hint_line = " ".repeat(position);
                    let hint_len = digit.len();
                    hint_line.push_str(&"^".repeat(hint_len));
                    print!("\n{}", &line[0..position]);
                    print!("\x1b[33m{}\x1b[0m", &line[position..position + digit.len()]);
                    println!("{}", &line[position + digit.len()..]);
                    println!("\x1b[33m{}\x1b[0m", hint_line);
                    println!("First is {} last is {}\n", first, last);
                }
            }
            None => {
                if debug {
                    println!("\x1b[31m{}\x1b[0m not in {}", digits[index], line);
                }
            }
        }
    }
    if debug {
        println!("\n    \x1b[32mDECISION: in {line} first is {first} and last is {last}\x1b[0m\n", line=line, first=first, last=last);
    }
    return first as u32 * 10 + last as u32;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;
    if args.len() > 1 {
        debug = true;
    }
    let file_contents = fs::read_to_string("input").expect("LogRocket: Should have been able to read the file");
    let mut total = 0;
    for token in file_contents.split('\n'){
        let number = get_line_number(&token, debug);
        total += number;
        if debug {
            println!("Token \x1b[32m{token}\x1b[0m adds \x1b[34m{number}\x1b[0m and total is \x1b[34m{total}\x1b[0m", token=token, total=total, number=number);
        }
        // break;
    }
    println!("The total of input is: \x1b[34m{}\x1b[0m", total);
}
