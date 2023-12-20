use aoc_input::input;
use std::collections::HashMap;

fn main() {
    let input: String = input::get_input_from_pwd();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}


fn part1(input: &str) -> usize {
    let tmp_input: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let pattern: &str = tmp_input[0];
    let mut map: HashMap<&str, (String, String)> = HashMap::new();

    for map_entry in tmp_input[1].lines() {
        let tmp: Vec<&str> = map_entry.split(" = ").collect::<Vec<&str>>();
        let key: &str = tmp[0];
        let lr_val: Vec<String> = tmp[1]
            .split(",")
            .map(|x| x.trim().replace(")", "")
                .replace("(", ""))
            .collect::<Vec<String>>();
        map.insert(key, (lr_val[0].clone(), lr_val[1].clone()));
    }

    let mut curr_node = "AAA";
    let mut steps = 0_usize;

    while curr_node != "ZZZ" {
        for letter in pattern.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>() {
            if letter == "R" {
                curr_node = &map.get(curr_node).unwrap().1;
            } else {
                curr_node = &map.get(curr_node).unwrap().0;
            }
            steps += 1;
            if curr_node == "ZZZ" {
                break;
            }
        }
    }
    steps
}


// TODO: Not working
fn part2(input: &str) -> usize {
    let tmp_input: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let pattern: &str = tmp_input[0];
    let mut entries: Vec<&str> = Vec::new();
    let mut map: HashMap<&str, (String, String)> = HashMap::new();

    for map_entry in tmp_input[1].lines() {
        let tmp: Vec<&str> = map_entry.split(" = ").collect::<Vec<&str>>();
        let key: &str = tmp[0];
        let lr_val: Vec<String> = tmp[1]
            .split(",")
            .map(|x| x.trim().replace(")", "")
                .replace("(", ""))
            .collect::<Vec<String>>();
        map.insert(key, (lr_val[0].clone(), lr_val[1].clone()));
        if key.ends_with("A") {
            entries.push(key);
        }
    }

    let mut curr_node = "AAA";
    let mut steps = 0_usize;

    // println!("{:?}", entries);

    while !entries.iter().map(|x| x.ends_with("Z")).all(|x| x) {
        for letter in pattern.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>() {
            for i in 0..entries.len() {
                if letter == "R" {
                    entries[i] = &map.get(entries[i]).unwrap().1;
                } else {
                    entries[i] = &map.get(entries[i]).unwrap().0;
                }
            }
            steps += 1;
            // println!("{:?}", entries);
            if entries.iter().map(|x| x.ends_with("Z")).all(|x| x) {
                break;
            }
        }
    }
    // println!("{:?}", map);
    // println!("{:?}", steps);
    steps
}

#[cfg(test)]
mod tests {
    fn test_input() -> String {
        String::from("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)")
    }

    #[test]
    fn part1() {
        let input = test_input();
        let result = super::part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2() {
        let input = test_input();
        let input = String::from("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        let result = super::part2(&input);
        assert_eq!(result, 6);
    }
}