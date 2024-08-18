// HashMap<K,V>
use std::{collections::HashMap, hash::Hash};
fn main() {
    // let mut scores: HashMap<String, u32> = HashMap::new();
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Bob"), 10);
    // scores.insert(10, 40);
    // let teams = vec![String::from("blue"), String::from("yellow")];
    // let initial_scores = vec![10, 50];
    // tuple  拉链作用 元组一对一
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // println!("{:#?}", scores);
    // {
    // "yellow": 50,
    // "blue": 10,

    //替代现有的值
    let mut scores = HashMap::new();
    scores.insert(String::from("Bob"), 10);
    scores.insert(String::from("Bob"), 20);
    // 检查K不对应值 时才插入V
    let e = scores.entry(String::from("Bob"));
    // 返回这个值的可变引用
    println!("{:?}", e);
    e.or_insert(30);

    let text = "hello hello world new";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map) //{"hello": 2, "world": 1, "new": 1}
}
