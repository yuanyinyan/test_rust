#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32)-> Rectangle{
        Rectangle{width: size, height: size}
    }
}

fn main() {
    let rest = Rectangle { width: 20, height: 30 };
    let rest1 = Rectangle { width: 10, height: 20 };
    let rest2 = Rectangle { width: 30, height: 40 };

    let area = rest.area();
    println!("area={}", area);
    println!("{}", rest.can_hold(&rest1));
    println!("{}", rest.can_hold(&rest2));

    println!("{:#?}", Rectangle::square(10));
}

//fn area(rest: &Rectangle) -> u32 {
//    rest.width * rest.height
//}
