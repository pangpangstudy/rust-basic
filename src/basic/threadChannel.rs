use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    // 通过克隆实现多个发送者
    // let tx_clone = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("hi"),
            String::from("hi"),
            String::from("hi"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    for received in rx {
        println!("{received}");
    }
    // let received = rx.recv().unwrap();
    // print!("{}", received);
}
