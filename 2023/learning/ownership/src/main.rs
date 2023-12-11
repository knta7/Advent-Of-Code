fn main() {
    //
    // take ownership
    //

    let string = String::from("testing");
    take_ownership(string);
    println!("{}", string);

    //
    // change
    //

    // let mut string = String::from("abc123");

    // change(&string);
    // change_mut(&mut string);
    // println!("{}", string)

    // let mut s = String::from("new string");
    //
    // let x = &s;
    // let y = &s;
    // println!("{}, {}", x, y);
    //
    // let z = &mut s;
    // println!("{}", z);

    //
    // Slices
    //

    // println!("{}", first_word(&String::from("test space")));
    //
    // let mut s = String::from("hello world");
    //
    // let word = first_word(&s);
    // println!("the first word is: {}", word);
    //
    // // s.clear(); // error!
    //
    // let s = "test world";
    //
    // let word = first_word(s);
    // println!("the first word is: {}", word);

    //
    // Test w int
    //

//
//   let mut y = 5;
//
//     let z = number(y);
//
//     println!("{}", z)

}

fn take_ownership(s : String) {
    println!("{}", s)
}

// fn change(s : &String) {
//     s.push_str("test")
// }

// fn change_mut(s : &mut String) {
//     s.push_str("test")
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn number(y: u32) -> u32 {
    y + 0
}