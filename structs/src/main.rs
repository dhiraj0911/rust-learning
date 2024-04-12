#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//Associated functions

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50
    };

    let rect1 = Rectangle{
        width: 10,
        height: 20
    };

    let rect2 = Rectangle{
        width: 50,
        height: 80
    };

    let rect3 = Rectangle::square(23);

    println!("This is from reactangle: {:#?}", rect);
    println!("Area of rectagle is {} sq m.", rect.area());
    println!("rect1 can hold {}", rect.can_hold(&rect1));
    println!("rect2 can hold {}", rect.can_hold(&rect2));

    println!("Area from associate function: {}", rect3.area());
}

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

