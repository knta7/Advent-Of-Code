use std::collections::HashMap;

fn main() {
    let integers: Vec<u32> = vec![1, 15, 23, 12, 14, 5, 6, 7, 8, 24, 0, 7, 35,71, 22, 3, 66, 51, 80, 95, 35, 8, 8, 35, 24, 8, 6, 4, 4, 8, 9, 34, 34];
    println!("Median: {}", get_median(&integers));
    println!("Mean: {}", get_average(&integers));
    println!("Mode: {}", get_mode(&integers));

    let test : &str = "d1";
    let t : usize = test.parse().expect("Unable to parse");
    println!("{}", t);
}

fn get_median(v: &Vec<u32>) -> u32 {
    let mut z = v.to_vec();
    z.sort();
    if let Some(val) = z.get(z.len()/ 2) {
        *val
    } else {
        0
    }
}

fn get_average(v: &Vec<u32>) -> f32 {
    let sum : u32 = v.iter().sum();
    sum as f32 / v.len() as f32
}

fn get_mode(v: &Vec<u32>) -> u32 {
    let mut occurrences : HashMap<u32, u32>= HashMap::new();
    let mut max_count: u32 = 0;
    for int in v {
        let entry = occurrences.entry(*int).or_insert(0);
        *entry += 1;
    }

    for (int, count) in &occurrences {
        if count > &occurrences.get(&max_count).copied().unwrap_or(0) {
            max_count = *int
        }
    }
    // println!("{:?}", occurrences);
    max_count
}