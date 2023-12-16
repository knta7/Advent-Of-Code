use aoc_input::input;

fn main() {
    let input: String = input::get_input_from_pwd();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Map {
    dest: usize,
    source: usize,
    range: usize,
}

impl Map {
    pub fn dest(&self) -> usize {
        self.dest
    }
    pub fn source(&self) -> usize {
        self.source
    }
    pub fn range(&self) -> usize {
        self.range
    }
}

fn part1(input: &str) -> usize {
    let inputs: Vec<&str> = input.split("\n\n").collect();
    let mut seeds: Vec<usize> = inputs[0].split(": ").collect::<Vec<&str>>()[1].split(" ")
        .map(|x| x.parse::<usize>().unwrap()).collect();

    let mut all_maps: Vec<Vec<Map>> = Vec::new();

    for i in 1..inputs.len() {
        let nums: Vec<usize> = inputs[i].split(":\n").collect::<Vec<&str>>()[1].replace("\n", " ").trim().split(" ")
            .map(|x| x.parse::<usize>().expect("Not sure why im breaking")).collect();
        let mut subset_maps: Vec<Map> = Vec::new();
        for y in 0..nums.len() / 3 {
            subset_maps.push(Map {
                dest: nums[y * 3],
                source: nums[y * 3 + 1],
                range: nums[y * 3 + 2],
            })
        }
        all_maps.push(subset_maps)
    }

    for i in &all_maps {
        for s in &mut seeds {
            *s = get_matching_value(&s, i);
        }
    }

    match seeds.iter().min() {
        Some(min) => *min,
        None => 0,
    }
}


fn part2(input: &str) -> usize {
    let inputs: Vec<&str> = input.split("\n\n").collect();
    let mut seeds: Vec<usize> = inputs[0].split(": ").collect::<Vec<&str>>()[1].split(" ")
        .map(|x| x.parse::<usize>().unwrap()).collect();
    let mut tmp_seeds: Vec<usize> = Vec::new();
    for i in 0..&seeds.len()/2 {
        for y in seeds[i*2]..seeds[i*2]+seeds[i*2+1] {
            tmp_seeds.push(y)
        }
    }
    seeds = tmp_seeds;

    let mut all_maps: Vec<Vec<Map>> = Vec::new();

    for i in 1..inputs.len() {
        let nums: Vec<usize> = inputs[i].split(":\n").collect::<Vec<&str>>()[1].replace("\n", " ").trim().split(" ")
            .map(|x| x.parse::<usize>().expect("Not sure why im breaking")).collect();
        let mut subset_maps: Vec<Map> = Vec::new();
        for y in 0..nums.len() / 3 {
            subset_maps.push(Map {
                dest: nums[y * 3],
                source: nums[y * 3 + 1],
                range: nums[y * 3 + 2],
            })
        }
        all_maps.push(subset_maps)
    }

    for i in &all_maps {
        for s in &mut seeds {
            *s = get_matching_value(s, i);
        }
    }

    match seeds.iter().min() {
        Some(min) => *min,
        None => 0,
    }
}

fn get_matching_value(int: &usize, map_list: &Vec<Map>) -> usize {
    for map in map_list {
        if map.source() <= *int && map.source() + map.range() - 1 >= *int {
            return map.dest() + (*int - map.source());
        }
    }
    *int
}

#[cfg(test)]
mod tests {

    fn test_input() -> String {
        String::from("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4")
    }
    #[test]
    fn part1() {
        let input = test_input();
        let result = super::part1(&input);
        assert_eq!(result, 35);
    }

    #[test]
    fn part2() {
        let input = test_input();
        let result = super::part2(&input);
        assert_eq!(result, 46);
    }
}