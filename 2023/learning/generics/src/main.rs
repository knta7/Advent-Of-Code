fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{}", largest(&numbers));
    println!("{:?}", largest_gen(&numbers));
}

fn largest(list : &[u32]) -> &u32 {
//     fn largest(list : &Vec<u32>) -> &u32 {
    let mut largest = &list[0];

    for ele in list {
        if ele > largest {
            largest = ele;
        }
    }
    largest
}

fn largest_gen<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for ele in list {
        if ele > largest {
            largest = ele;
        }
    }
    largest
}