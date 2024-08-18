#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let v = vec![1, 2, 3, 4];
    // let third = &v[100];
    // println!("{third}");
    match v.get(100) {
        Some(third) => println!("{third}"),
        None => println!("None"),
    }
    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(10.0),
        SpreadsheetCell::Text(String::from("Text")),
    ];
    println!("{:?}", row);
}
