fn main() {
    let number_list = vec![34, 35, 36, 37, 38];
    let r = largest(&number_list);
    println!("{}", r);
}
// 这里会报错  需要定义好类型 后边会学到
fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // 模式匹配  &[i32] == &number ---> number == i32
    for number in list.iter() {
        if number > &largest {
            largest = number;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 35, 36, 37, 38];
    let r = largest(&number_list);
    println!("{}", r);
}
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    // 模式匹配  &[i32] == &number ---> number == i32
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
