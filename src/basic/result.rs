use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    // let e = vec![1];
    // println!("{}", e[100]);
    let r = read_username_from_file();
    println!("{:?}", r.expect("失败 "));
}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // 如果OK就继续执行 如果失败就return   ？ --> from函数处理 错误转换
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
// fn read_username_from_file() -> Result<String, io::Error> {
//     // 如果OK就继续执行 如果失败就return   ？ --> from函数处理 错误转换
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
