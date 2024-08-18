fn main() {
    let _tup: (u128, bool) = (12, false);
    // println!("{}", tup.0);
    // let a: [u128; 4] = [1, 3, 4, 5]; //array
    let _b = [3; 5]; //array

    // println!("{}", b[7]); //编译不报错  运行时报错

    // let x = five();
    // println!("{x}")
    // _ctrl();
    _str_test();
}
fn _fun1() {
    let _x = 5;
    let y = {
        let x = 1;
        // x + 3;
        x + 3 //无分号 代表块返回值
    };
    println!("Y:{}", y);
}
fn _five() -> i32 {
    // let number = if false { 7 } else { "6" }; error
    let number = if false { 7 } else { 6 };
    println!("{number}");
    5;
    return 16;
}
// loop while for
fn _ctrl() {
    // loop {
    //     println!("test loop");
    //     break;
    // }

    // while 1 != 2 {}
    // let _b = [1, 2, 3, 4, 5, 6];
    // for value in _b {
    //     println!("value:{}", value);
    // }
    let _b = [1, 2, 3, 4, 5, 6];
    // &i32
    for value in _b.iter() {
        println!("value:{}", value);
    }
    for value in (1..4).rev() {
        println!("value:{}", value);
    }
}
fn _str_test() {
    let mut a = "hello";
    a = "new"; //直接覆盖？
    let mut b = String::from("hello");
    b.push_str(",world");
    println!("a:{a} \nb:{b}");
}
// Move Copy trait clone
// 为防止二次释放bug,s2复制s1指针之后 s1废弃
// s2离开作用域-内存释放 s1被废弃 不会再次释放相同的heap 内存数据
// s1 指针 len cap 废弃
// s2 指针 len cap
// fn _double_re() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     let s2 = s1.clone();
//     println!("{s1}")
// }
// 所有权的转移 会导致原先变量的废弃 rust 引用
// 借用：把引用作为函数参数的行为
// 引用者  离开作用域的时候 不会释放被引用的内存  因为其没有所有权
/*
fn _re() {
    let s1 = String::from("hello");
    let len = _c(&s1);
    println!("{len}")
}
fn _c(s: &String) -> usize {
    s.len()
}
 可变引用
fn _c(s: &mut String) -> usize {
    s.len()
}
*/
