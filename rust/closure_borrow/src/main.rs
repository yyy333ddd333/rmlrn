use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    //let v2 = 5;
    //let v_ref = &vec![4, 5, 6];
    //println!("{:?}", v_ref);
    
    let handle = thread::spawn(move || {
        //2 * v2
        //println!("{:?}", v2);
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    
}
