use std::io;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+|\d+\.\d+)[cfCF]$").unwrap();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input = input.trim().to_owned();

        match re.is_match(&input) {
            true => {
                let unit = input[input.len() - 1..].to_string().to_uppercase();
                match input[0..input.len() - 1].parse::<f32>() {
                    Ok(num) => {
                        match unit.as_str() {
                            "C" => {
                                println!("{}F", (num * 9.0 / 5.0) + 32.0)
                            }
                            "F" => {
                                println!("{}C", (num - 32.0) * 5.0 / 9.0)
                            }
                            _ => {
                                println!("None")
                            }
                        }
                    },
                    Err(_) => {
                        println!("Unable to parse");
                    }
                };
            }
            false => {
                println!("Invalid entry");
            }
        }
    }
}
