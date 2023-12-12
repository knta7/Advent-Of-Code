use aoc_input::input;

fn main() {
    let mut input: String = input::get_input_from_pwd();
//     input = String::from("467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..");

    let str_symbols = "!@#$%^&*()_+-=/";
    let vec_symbols: Vec<char> = str_symbols.chars().collect::<Vec<char>>();

    let mut vec_data: Vec<(usize, usize, usize, u64)> = Vec::new();


    let mut data: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        data.push(line.trim().chars().collect::<Vec<char>>());
    }

    for (pos_row, row) in data.iter().enumerate() {
        for (pos_col, sym) in row.iter().enumerate() {
            if vec_symbols.contains(&sym) {
                for x in -1..2 {
                    for y in -1..2 {
                        let mut check_row: usize = (pos_row as i64 + x) as usize;
                        let mut check_col: usize = (pos_col as i64 + y) as usize;
                        if check_row >= 0 && check_col >= 0 && check_row <= row.len() && check_col <= data.len() {
                            if let Some(tmp_row) = data.get(check_row) {
                                if let Some(tmp_val) = tmp_row.get(check_col) {
                                    if tmp_val.is_numeric() {
                                        let mut val_start = check_row;
                                        let mut val_end = tmp_row.len();
                                        for i in (0..check_col+1).rev() {
                                            if let Some(val) = tmp_row.get(i) {
                                                if val.is_numeric() {
                                                    val_start = i;
                                                } else {
                                                    break;
                                                }
                                            }
                                        }
                                        for i in check_col..tmp_row.len() {
                                            if let Some(val) = tmp_row.get(i) {
                                                if !val.is_numeric() {
                                                    val_end = i;
                                                    break;
                                                }
                                            }
                                        }
                                         let chars_slice = &tmp_row[val_start..val_end];
                                        let full_num: u64 = chars_slice.iter().cloned().collect::<String>().parse().expect("Cannot parse");
                                        vec_data.push((check_row, val_start, val_end, full_num))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vec_data.sort();
    vec_data.dedup();

    let mut total = 0u64;
    for ele in &vec_data {
        total += ele.3;
    }
    println!("Part 1: {:?}", total);

    let mut total = 0u64;
    for (pos_row, row) in data.iter().enumerate() {
        for (pos_col, sym) in row.iter().enumerate() {
            if *sym == '*' {
                let mut vec_data: Vec<(usize, usize, usize, u64)> = Vec::new();
                for x in -1..2 {
                    for y in -1..2 {
                        // println!("{}, {}", x, y);
                        let mut check_row: usize = (pos_row as i64 + x) as usize;
                        let mut check_col: usize = (pos_col as i64 + y) as usize;
                        if check_row >= 0 && check_col >= 0 && check_row <= row.len() && check_col <= data.len() {
                            if let Some(tmp_row) = data.get(check_row) {
                                if let Some(tmp_val) = tmp_row.get(check_col) {
                                    if tmp_val.is_numeric() {
                                        let mut val_start = check_row;
                                        let mut val_end = tmp_row.len();
                                        for i in (0..check_col+1).rev() {
                                            if let Some(val) = tmp_row.get(i) {
                                                if val.is_numeric() {
                                                    val_start = i;
                                                } else {
                                                    break;
                                                }
                                            }
                                        }
                                        // need to use
                                        for i in check_col..tmp_row.len() {
                                            if let Some(val) = tmp_row.get(i) {
                                                if !val.is_numeric() {
                                                    val_end = i;
                                                    break;
                                                }
                                            }
                                        }
                                        let chars_slice = &tmp_row[val_start..val_end];
                                        let full_num: u64 = chars_slice.iter().cloned().collect::<String>().parse().expect("Cannot parse");
                                        vec_data.push((check_row, val_start, val_end, full_num))
                                    }
                                }
                            }
                        }
                    }
                }
                vec_data.sort();
                vec_data.dedup();
                if vec_data.len() == 2 {
                    total += vec_data[0].3 * vec_data[1].3;
                }
            }
        }
    }

    println!("Part 2: {:?}", total);
}
