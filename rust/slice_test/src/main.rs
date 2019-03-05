fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v[0] = 6;
    //let v1: &mut [i32] = &mut v[0];
    v.push(6);
    println!("{},{}", v[0], v[5]);
    let mut s1 = String::from("foo");

    let s2 = "bar";//字符字面量是&str引用，不涉及所有权问题
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s = String::from("hello world");
    let hello = &s[0..5];//slices的大小只有在运行时才知道，所以一般变量都指向&引用
    let world = &s[6..11];
    println!("{},{}", hello, world);

    let x = 1;
    let y = x;
    println!("{},{}", x, y);
}
