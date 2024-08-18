#[derive(Debug)]
struct Square {
    width: u32,
    height: u32,
}
impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Square) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn square(size: u32) -> Square {
        Square {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let test = Square {
        width: 30,
        height: 40,
    };
    let test1 = Square {
        width: 10,
        height: 40,
    };
    let test2 = Square {
        width: 40,
        height: 50,
    };
    let test3 = Square {
        width: 30,
        height: 60,
    };
    let s = Square::square(24);
    println!("{}", test.can_hold(&test1));
    println!("{}", test.can_hold(&test2));
    println!("{}", test.can_hold(&test3));
    // println!("{}", test);
    // println!("{:?}", test);
    // println!("{:#?}", test);
    println!("{:#?}", s);
}
