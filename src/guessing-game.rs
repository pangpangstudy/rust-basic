use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("猜数字！");
    let secret_number = rand::thread_rng().gen_range(0..10);
    println!("{secret_number}");
    loop {
        let mut guess_number = String::new();
        println!("请输入一个数字！");
        io::stdin()
            .read_line(&mut guess_number)
            .expect("无法读取行");
        // let guess_number: u32 = guess_number.trim().parse().expect("请输入数字类型");
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是 {}", guess_number);

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
