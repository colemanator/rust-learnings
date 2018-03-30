struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn can_hold (&self, rectangle: &Rectangle) -> bool {
        return self.width > rectangle.width && self.height > rectangle.height
    }

    fn square (size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main () {
    let rect = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pxiels",
        rect.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can react hold rect2 {}", rect.can_hold(&rect2));
    println!("Can react hold rect3 {}", rect.can_hold(&rect3));
}

