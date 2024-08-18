fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    // fn longest(x: &str, y: &str) -> &str
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

// 主要目标避免悬垂引用
/*
fn main() {

    // 原因：被引用对象x的生命周期 要比 引用者短
    let r;
    {
        let x = 5;
        // borrowed value does not live long enough
        r = &x;
    } //这里x走出作用域`x` dropped here while still borrowed 内存释放
      // 这里x的数据被释放 报错  -----> 借用检查器
    println!("r:{}", r);
}
*/
