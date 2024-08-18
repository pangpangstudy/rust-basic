// 计算长方形面积
#[derive(Debug)]
struct Square {
    width: u32,
    height: u32,
}
fn main() {
    let test = Square {
        width: 30,
        height: 40,
    };
    println!("{}", area(&test));
    // println!("{}", test);
    // println!("{:?}", test);
    println!("{:#?}", test);
}
fn area(square: &Square) -> u32 {
    square.width * square.height
}
/*
fn main() {
    let rect = (30,40);
    println!("{}", area(rect));
}
fn area(dim:(u32,u32)) -> u32 {
    dim.0 * dim.1
}
*/
/*
fn main() {
    let w = 30;
    let h = 20;
    println!("{}", area(w, h));
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/
