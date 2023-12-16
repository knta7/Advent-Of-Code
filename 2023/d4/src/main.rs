use aoc_input::input;

fn main() {
    let mut input: String = input::get_input_from_pwd();
//     input = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");


    let mut total = 0u64;

    for line in input.lines() {
        let hands: Vec<&str> = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect();
        let nums_win: Vec<&str> = hands[0].split(" ").collect();
        let nums_choose: Vec<&str> = hands[1].split(" ").collect();

        let mut pwr = -1;
        for cho in nums_choose {
            if cho != "" {
                if nums_win.contains(&cho) {
                    pwr += 1;
                }
            }
        }
        if pwr >= 0 {
            total += (2i64.pow(pwr as u32)) as u64;
        }
    }

    println!("Part 1: {}", total);

    let mut card_points: Vec<usize> = Vec::new();
    let mut card_counter: Vec<usize> = Vec::new();

    for line in input.lines() {
        let tmp_str = line.to_string();
        let tmp: Vec<&str> = tmp_str.split(": ").collect();
        let hands: Vec<&str> = tmp[1].split(" | ").collect();
        let nums_win: Vec<&str> = hands[0].split(" ").collect();
        let nums_choose: Vec<&str> = hands[1].split(" ").collect();

        let mut matches = 0;
        for cho in nums_choose {
            if cho != "" && nums_win.contains(&cho) {
                matches += 1;
            }
        }
        card_points.push(matches);
        card_counter.push(1);
    }

    for id in 0..card_counter.len() {
        for i in id + 1..card_points[id] + id + 1 {
            if i <= card_counter.len() - 1 {
                card_counter[i] = card_counter[i] + card_counter[id];
            } else {
                break;
            }
        }
    }

    let total: usize = card_counter.iter().sum();
    println!("Part 2: {}", total);
}
