#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// definindo um metodo
impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.area() > other.area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    let rect4 = Rectangle::square(4);

    println!(
        "The area of the rectangle is {} square pixels",
        rect4.area()
    );

}
