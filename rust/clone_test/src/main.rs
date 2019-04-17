fn main() {
    let a = Test{
        i: 0,
        s: String::from("test"),
    };
    test(&a);
}

struct Test {
    i: i32,
    s: String,
}

//error[E0507]: cannot move out of borrowed content
//fn test(s: &Test) -> Box<Test> {
fn test(s: &Test) -> Test {
    *s.clone()
    //Box::new(*s.clone())
    //Box::new(b)
}
/*
fn test<'a>(s: &'a Test) -> Box<'a, Test> {
    Box::new(*s.clone())
}
*/
