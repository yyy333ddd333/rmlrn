use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    //let v2 = 5;
    //let v_ref = &vec![4, 5, 6];
    //println!("{:?}", v_ref);
    
    //这里用move是因为如果不用move，printlin!会借用&v作为参数，导致生命周期和主函数的生命周期一致。但是spawn()要求闭包参数的生命周期是'static
    let handle = thread::spawn(move || {
        //2 * v2
        //println!("{:?}", v2);
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    
}
