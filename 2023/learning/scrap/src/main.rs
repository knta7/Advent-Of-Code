fn main() {
    // let r;
    //
    // r = create_dangle();
    // println!("{}", r);
    // println!("{:?}", longest(r))

    let x : Vec<&str> = "12,31,51,25,1,2,3,1,2,3".split(",").collect::<Vec<&str>>();
    let y : Vec<usize> = "12,31,51,25,1,2,3,1,2,3,0".split(",")
        .map(|x| x.parse::<usize>().expect("Not sure why im breaking")).collect();
}

fn create_dangle() -> &'static str {
    let x: &'static str = "test";
    x
}

// Between function name and parameter parameter list is where you declare generics to be used
// lifetimes ('a) or parameters (P, T, etc)
fn longest<'a>(x: &'a str) {
    let x: &'static str = "test";
    // x
}