use aoc_input::input;

fn main() {
    let mut input: String = input::get_input_from_pwd();
//     input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    let str_limits = String::from("12 red,13 green,14 blue");
    let mut limits: Vec<&str> = str_limits.split(",").collect();
    let mut total = 0u64;
    for line in input.lines() {
        let tmp = line.to_string();
        let elements: Vec<&str> = tmp.split(": ").collect();
        let id: &str = elements[0];
        let id: u32 = id.replace("Game ", "").parse().expect("Unable to parse");

        let raw_hands: Vec<&str> = elements[1].split("; ").collect();

        let mut pass = true;
        for hand in raw_hands {
            // println!("{:?}", hand);
            for roll in hand.split(", ").collect::<Vec<&str>>() {
                // println!("{:?}", roll);
                let amt: u32 = roll.split(" ").collect::<Vec<&str>>()[0].parse().expect("Cannot parse");
                let color: &str = roll.split(" ").collect::<Vec<&str>>()[1];
                let limit_amt = get_limits(color, &limits);
                if amt > limit_amt {
                    pass = false;
                }
            }
        }
        if pass {
            total += id as u64;
        }
    }
    println!("Part 1: {}", total);

    let mut total = 0u64;

    for line in input.lines() {
        let tmp = line.to_string();
        let elements: Vec<&str> = tmp.split(": ").collect();
        let raw_hands: Vec<&str> = elements[1].split("; ").collect();

        let mut min_red = 0u32;
        let mut min_blue = 0u32;
        let mut min_green = 0u32;
        for hand in raw_hands {
            for roll in hand.split(", ").collect::<Vec<&str>>() {
                let amt: u32 = roll.split(" ").collect::<Vec<&str>>()[0].parse().expect("Cannot parse");
                let color: &str = roll.split(" ").collect::<Vec<&str>>()[1];
                match color {
                    "red" => {
                        if min_red < amt {
                            min_red = amt;
                        }
                    }
                    "green" => {
                        if min_green < amt {
                            min_green = amt;
                        }
                    }
                    "blue" => {
                        if min_blue < amt {
                            min_blue = amt;
                        }
                    }
                    &_ => ()
                }
            }
        }
        total += (min_red * min_green * min_blue) as u64;
    }
    println!("Part 2: {}", total);
}

fn get_limits(desired_color: &str, limits: &Vec<&str>) -> u32 {
    for limit in limits {
        let amt: u32 = limit.split(" ").collect::<Vec<&str>>()[0].parse().expect("Cannot parse");
        let color: &str = limit.split(" ").collect::<Vec<&str>>()[1];
        if color == desired_color {
            return amt;
        }
    }
    0
}
