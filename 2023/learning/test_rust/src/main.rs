fn main() {
    // let y: u8 = 16;
    // let z = &y;
    //
    // let yy = String::from("test");
    // let zz = yy.clone();
    //
    // let yyy: String = String::from("tester");
    // let zzz = cp(&yyy);
    //
    // println!("{}", y + z);
    // println!("{}", yy + &zz);
    // println!("{}", yyy + zzz.to_string().as_str());

    // let v = vec![0, 1, 2, 3 ,4];
    //
    // println!("{:?}", v);
    // for i in &v {
    //     println!("{i}")
    // }
    // println!("{:?}", v);

    // Cannot run because Unicode scalar values take up 2 bytes of storage per character
    // When stored in Vec<u8>, the indices don't match the characters
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
}

fn cp(input: &String) -> usize {
    input.len()
}