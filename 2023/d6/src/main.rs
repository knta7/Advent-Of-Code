use aoc_input::input;

fn main() {
    let input: String = input::get_input_from_pwd();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let races: Vec<Race> = decode_input(input);
    let mut result = 1usize;
    for race in races {
        result *= race.ways_to_win().len();
    }
    result
}

fn part2(input: &str) -> usize {
    let races: Vec<Race> = decode_input2(input);
    let mut result = 1usize;
    for race in races {
        result *= race.ways_to_win().len();
    }
    result
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn ways_to_win(&self) -> Vec<u64> {
        // self.time
        // vec![5,10]
        let mut out: Vec<u64> = Vec::new();
        for i in 1..self.time {
            if (self.time - i) * i > self.distance {
                out.push(i)
            }
        }
        out
    }
}

fn decode_input(input: &str) -> Vec<Race> {
    let mut out: Vec<Race> = Vec::new();
    let inputs: Vec<Vec<u64>> = input.split("\n")
        .map(|line| line.trim_start_matches("Time:")
            .trim_start_matches("Distance:").trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|val| val.parse::<u64>()
                .unwrap())
            .collect::<Vec<u64>>()
        ).collect::<Vec<Vec<u64>>>();

    for i in 0..inputs[0].len() {
        let race: Race = Race {
            time: inputs[0][i],
            distance: inputs[1][i],
        };
        out.push(race);
    }
    out
}

fn decode_input2(input: &str) -> Vec<Race> {
    let mut out: Vec<Race> = Vec::new();
    let inputs: Vec<Vec<u64>> = input.split("\n")
        .map(|line| line.trim_start_matches("Time:")
            .trim_start_matches("Distance:").replace(" ", "")
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|val| val.parse::<u64>()
                .unwrap())
            .collect::<Vec<u64>>()
        ).collect::<Vec<Vec<u64>>>();

    for i in 0..inputs[0].len() {
        let race: Race = Race {
            time: inputs[0][i],
            distance: inputs[1][i],
        };
        out.push(race);
    }
    out
}

#[cfg(test)]
mod tests {
    fn test_input() -> String {
        String::from("Time:      7  15   30
Distance:  9  40  200")
    }

    #[test]
    fn part1() {
        let input = test_input();
        let result = super::part1(&input);
        assert_eq!(result, 288);
    }

    #[test]
    fn part2() {
        let input = test_input();
        let result = super::part2(&input);
        assert_eq!(result, 71503);
    }
}