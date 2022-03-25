#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("The area of the rectangle is {} square pixel", rect1.area());
    println!("Rect1 can hold Rect2: {}", rect1.can_hold(&rect2));
    println!("Rect1 can hold Rect3: {}", rect1.can_hold(&rect3));
}
