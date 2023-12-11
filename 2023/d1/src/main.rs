use aoc_input::input;

fn main() {
    let mut input: String = input::get_input_from_pwd();
//     input = String::from("1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet");

    let mut calibration_values: Vec<u32> = Vec::new();
    for line in input.lines() {
        // let chars : Vec<&str> = line.trim().split("").collect();
        let chars: Vec<char> = line.trim().chars().collect();
        // println!("{:?}", chars);

        let mut first_char = String::new();
        let mut last_char = String::new();
        for chr in chars {
            // println!("{}", chr.is_numeric());
            if chr.is_numeric() {
                if first_char == String::new() {
                    first_char = chr.to_string();
                }
                last_char = chr.to_string();
            }
        }

        let val: u32 = format!("{first_char}{last_char}").parse().expect("Unable to parse combined chars");
        calibration_values.push(val);
    }
    let sum: u32 = calibration_values.iter().sum();
    println!("Part 1: {:?}", sum);

//     input = String::from("two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen");

    let mut calibration_values: Vec<u32> = Vec::new();
    let str_numbers = String::from("one,two,three,four,five,six,seven,eight,nine");
    let numbers: Vec<&str> = str_numbers.split(",").collect();
    for line in &mut input.lines() {
        let mut vec_idx: Vec<usize> = Vec::new();
        let mut chars : Vec<char> = line.trim().chars().collect();
        for (pos, str_num) in numbers.iter().enumerate() {
            // mut_line = mut_line.replace(str_num, format!("{}", pos + 1).as_str())
            if let Some(idx) = line.find(str_num) {
                vec_idx.push(idx);
                chars[idx] = char::from_digit( (pos + 1) as u32, 10).unwrap();
            }
        }

        let mut first_char = String::new();
        let mut last_char = String::new();
        for chr in &chars {
            // println!("{}", chr.is_numeric());
            if chr.is_numeric() {
                if first_char == String::new() {
                    first_char = chr.to_string();
                }
                last_char = chr.to_string();
            }
        }

        let val: u32 = format!("{first_char}{last_char}").parse().expect("Unable to parse combined chars");
        calibration_values.push(val);

        // println!("{:?}", vec_idx);
        // println!("{:?}", chars);
    }
    // println!("{:?}", input);
    let sum: u32 = calibration_values.iter().sum();
    println!("Part 2: {:?}", sum);
}
