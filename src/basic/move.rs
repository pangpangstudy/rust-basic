use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    // 所有权转移
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // println!("Here's a vector: {:?}", v); //报错 所有权已经不在
    // drop(v)
    handle.join().unwrap();
}
