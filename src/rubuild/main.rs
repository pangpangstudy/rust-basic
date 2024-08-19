use std::{env, process};

use rust_basic::Config;

fn main() {
    // collect将迭代器转为一个集合
    // let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = rust_basic::run(config) {
        eprintln!("App error: {}", e);
        process::exit(1);
    };
}
