use std::{fs, path::Path, env};
use reqwest::header::COOKIE;

pub mod input {
    pub fn get_input_from_pwd() -> String {
        get_input(&mut 0, &mut 0)
    }

    pub fn get_input(year: &mut u32, day: &mut u32) -> String{
        if *year == 0 || *day == 0 {
            println!("{}", get_current_working_dir());
            (*year, *day) = get_current_year_day();
        }

        let input_path: &str = "../inputs";
        if !super::Path::new(format!("{input_path}/{day}").as_str()).exists() {
            println!("Input {day} does not exist");
            let cookie: &str = "../sessionCookie";
            if super::Path::new(cookie).exists() {
                let session: String = super::fs::read_to_string(cookie)
                    .expect("Should have been able to read the file");

                let _ = super::fs::create_dir_all(input_path);
                if !super::Path::new("{input_path}/{day}").exists() {
                    request(year, day, &session, &input_path)
                } else {
                    String::new()
                }
            } else {
                println!("Missing {cookie}");
                String::new()
            }
        } else {
            println!("Input {day} already exists");
            match super::fs::read_to_string(format!("{input_path}/{day}")) {
                Ok(content) => {
                    content
                }
                Err(e) => {
                    println!("{e} error occurred");
                    String::new()
                }
            }
        }
    }

    fn request(year: &u32, day: &u32, session: &str, input_path: &str) -> String {
        let cookie = format!("session={session}");
        let client = reqwest::blocking::Client::new();
        if let Ok(result) = client.get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header(super::COOKIE, cookie)
            .send() {
            if let Ok(input) = result.text() {
                println!("Creating input file {day}");
                super::fs::write(format!("{input_path}/{day}"), &input).expect("Unable to write file");
                input
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    pub fn get_current_working_dir() -> String {
        super::env::current_dir().unwrap().to_str().unwrap().to_string()
    }

    pub fn get_current_year_day() -> (u32, u32) {
        let path = get_current_working_dir();
        // printing path shows \ as \\
        let parts: Vec<&str> = path.split(r"\").collect();
        // add in logic to prevent panic when no element found
        let year: u32 = parts.get(parts.len() - 2).unwrap().parse().unwrap();
        let day: u32 = parts.get(parts.len() - 1).unwrap()[1..].parse().unwrap();
        (year, day)
    }
}
