#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation of struct
// Multiple implementations possible
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Function create new instance of self
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn method_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(3); // create new instance
}
