use std::collections::HashMap;
use std::cell::RefCell;
fn main() {
    let mut h1 = HashMap::new();
    h1.insert(1, "a");
    let h2 = RefCell::new(HashMap::new());
    h2.borrow_mut().insert(2, "b");
    println!("{:?}", h2.borrow());
}
