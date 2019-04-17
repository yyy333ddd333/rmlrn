fn main() {
    let mut a = String::from("a");
    let b = &a;
    a = String::from("b");
    println!("{}", b);
}
