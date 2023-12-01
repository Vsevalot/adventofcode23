use std::fs;
fn main() {
    let file_contents = fs::read_to_string("input").expect("LogRocket: Should have been able to read the file");
    let mut total = 0;
    for token in file_contents.split('\n'){
        let mut first_found = false;
        let mut first = 0;
        let mut last = 0;
        for c in token.chars(){
            if c.is_digit(10) {
                let digit = c as u32 - '0' as u32;
                if !first_found {
                    first_found = true;
                    first = digit;
                }
                last = digit;
            }
        }
        total = total + first * 10 + last;
        println!("After token {} total is {}", token, total);
    }
}
