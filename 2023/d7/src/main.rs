use aoc_input::input;

fn main() {
    let input: String = input::get_input_from_pwd();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut total = 0usize;

    let mut hands: Vec<Hand> = Vec::new();

    for ele in input.lines() {
        let tmp: Vec<&str> = ele.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();
        let hand: &str = tmp[0];
        // Figure out type
        let mut matches: Vec<usize> = Vec::new();
        for card in hand.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>() {
            match hand.matches(card).count() {
                1 => {}
                num => { matches.push(num) }
            }
        }

        let hand_type = match matches.len() {
            2 => 2,
            3 => 4,
            // Check for 4x1 and 2x2
            4 => match matches[0] {
                2 => 3,
                _ => 6,
            },
            // Check for 5x1 and 3x1 + 2x1
            5 => match matches[0] {
                5 => 7,
                _ => 5,
            },
            _ => 1,
        };

        hands.push(Hand {
            hand: tmp[0],
            points: tmp[1].parse().expect("Unable to parse"),
            hand_type,
        });
    }

    sort(&mut hands);

    for (i, hand) in hands.iter().enumerate() {
        // println!("{}, {}, {}, {}", hand.hand, i + 1, hand.points, hand.hand_type);
        total += hand.points * (i + 1);
    }
    total
}

// TODO: Does not work, need to fix
// Issues are using J only once and to use it only for the largest combo possible
// Band-aid code breaks logic in other areas
fn part2(input: &str) -> usize {
    let mut total = 0usize;

    let mut hands: Vec<Hand> = Vec::new();

    for ele in input.lines() {
        let tmp: Vec<&str> = ele.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();
        let hand: &str = tmp[0];
        // Figure out type
        let mut matches: Vec<usize> = Vec::new();
        let mut j_count: usize = hand.matches("J").count();
        for card in hand.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>() {
            let mut match_val: usize = hand.matches(card).count();
            if card == "J" {
                match_val = 0;
            }
            // if hand == "J462A" {
            //     println!("{}, {}", hand.matches(card).count(), j_count);
            // }
            // println!("Match Val {}", match_val);
            matches.push(match_val)
        }

        // println!("{}", matches.len());
        if j_count > 0 {
            let mut max_idx = 0;
            for i in 1..matches.len() {
                if matches[max_idx] < matches[i] {
                    max_idx = i;
                }
            }
            matches[max_idx] += j_count;
        }

        // if hand == "J2828" {
        //     println!("{:?}", matches);
        // }
        let hand_type = match matches.iter().max() {
            Some(max) => {
                match max {
                    5 => 7,
                    4 => 6,
                    3 => {
                        match matches.len() {
                            1 => 4,
                            _ => 5,
                        }
                    }
                    2 => {
                        match matches.iter().filter(|&v| *v == 2).count() {
                            1 => 2,
                            2 => 2,
                            3 => 2,
                            _ => 3,
                        }
                    }
                    _ => 1,
                }
            }
            None => 1,
        };
        // if hand == "J462A" {
        //     println!("{}", hand_type);
        // }

        hands.push(Hand {
            hand: tmp[0],
            points: tmp[1].parse().expect("Unable to parse"),
            hand_type,
        });
    }

    // println!("{:?}", hands);
    sort2(&mut hands);
    // println!("");
    // println!("{:?}", hands);

    for (i, hand) in hands.iter().enumerate() {
        // println!("{}", hand.hand);
        // println!("{}, {}, {}, {}", hand.hand, i + 1, hand.points, hand.hand_type);
        total += hand.points * (i + 1);
    }
    total
}

#[derive(Debug, Copy, Clone)]
struct Hand<'a> {
    hand: &'a str,
    points: usize,
    hand_type: u8,
}

impl Hand<'_> {
    // true = self is larger, false = self is smaller
    // 1 - 7 indicates hand type; 1 = high card; 7 = 5 of a kind
    pub fn gt(&self, other: Hand) -> bool {
        match self.hand_type > other.hand_type {
            true => true,
            _ => match self.hand_type == other.hand_type {
                true => {
                    let cards: Vec<&str> = "AKQJT98765432".split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
                    let self_hand: Vec<&str> = self.hand.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
                    let other_hand: Vec<&str> = other.hand.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();

                    for i in 0..self_hand.len() {
                        let comparison: i64 = get_card_idx(&self_hand[i], &cards).unwrap()
                            - get_card_idx(&other_hand[i], &cards).unwrap();
                        if comparison < 0 {
                            return true;
                        }
                        if comparison > 0 {
                            return false;
                        }
                    }
                    return false;
                }
                _ => false,
            }
        }
    }

    pub fn gt_2(&self, other: Hand) -> bool {
        match self.hand_type > other.hand_type {
            true => true,
            _ => match self.hand_type == other.hand_type {
                true => {
                    let cards: Vec<&str> = "AKQT98765432J".split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
                    let self_hand: Vec<&str> = self.hand.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
                    let other_hand: Vec<&str> = other.hand.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();

                    for i in 0..self_hand.len() {
                        let comparison: i64 = get_card_idx(&self_hand[i], &cards).unwrap()
                            - get_card_idx(&other_hand[i], &cards).unwrap();
                        if comparison < 0 {
                            return true;
                        }
                        if comparison > 0 {
                            return false;
                        }
                    }
                    return false;
                }
                _ => false,
            }
        }
    }
}

fn get_card_idx(card: &str, cards: &Vec<&str>) -> Option<i64> {
    for (i, crd) in cards.iter().enumerate() {
        if *crd == card {
            return Some(i as i64);
        }
    }
    None
}

fn sort(hands: &mut Vec<Hand>) {
    for i in 0..hands.len() {
        for j in 0..hands.len() - i - 1 {
            if hands[j].gt(hands[j + 1]) {
                hands.swap(j, j + 1);
            }
        }
    }
}

fn sort2(hands: &mut Vec<Hand>) {
    for i in 0..hands.len() {
        for j in 0..hands.len() - i - 1 {
            if hands[j].gt_2(hands[j + 1]) {
                hands.swap(j, j + 1);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    fn test_input() -> String {
        String::from("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483")
    }

    #[test]
    fn part1() {
        let input = test_input();
        let result = super::part1(&input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part2() {
        let input = test_input();
        let result = super::part2(&input);
        assert_eq!(result, 5905);
    }
}