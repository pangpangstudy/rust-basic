use std::fmt::{Debug, Display};

/*
Trait
Trait告诉Rust编译器：某种类型具有哪些并且可以与其他类型共享的功能
抽象的定义共享行为
Trait bounds：反省类型参数指定为实现了特定行为的类型
Trait的定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为。
- 关键字:trait
- 只有方法签名，没有具体实现
- trait 可以有多个方法:每个方法签名占一行，以;结尾
- 实现该trait的类型必须提供具体的方法实现
*/
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}
// 简单情况
pub fn notify(item: impl Summary) {}
pub fn notify3(item: impl Summary + Display) {}
// 复杂情况
pub fn notify2<T: Summary>(item: T, item1: T) {}
pub fn notify4<T: Summary + Display>(item: T, item1: T) {}
pub fn notify5<T: Summary + Display, U: Clone + Debug>(item: T, item1: U) {}
// where子句
pub fn notify6<T, U>(item: T, item1: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    String::new()
}
// trait作为返回值
/*
pub fn notify7(flag: bool) -> impl Summary {
    // 这里就报错 因为只能返回一种类型的可能
    if flag {
        NewArticle {
            headline: String::new(),
            content: String::new(),
            author: String::new(),
            location: String::new(),
        }
    } else {
        Tweet {
            username: String::from("horse ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/
// 在类型上实现trait
fn main() {
    let tweet = Tweet {
        username: String::from("horse ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet:{}", tweet.summarize())
}
