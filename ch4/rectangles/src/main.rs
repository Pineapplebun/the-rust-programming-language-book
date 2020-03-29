fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect3 = Rectangle::square(40);
    println!("The area of the rectangle {} square pixels.", rect1.area());
    println!("The rect is {:#?}", rect1);
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect3? {}", rect2.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(length: u32) -> Rectangle {
        Rectangle {
            width: length,
            height: length,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
