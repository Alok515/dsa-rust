struct Rectangle {
    width: u32,
    height: u32
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn print_area( area: u32) {
    println!("The area of the rectangle is {} square pixels.", area);
}

fn can_hold_print( rect_a: &str, react_b: &str, hold: bool) {
    println!("Can {} hold {} = {}", rect_a, react_b, hold);
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect1_area = rect1.area();
    print_area(rect1_area);

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle::square(25);

    can_hold_print("rect1", "rect2", rect1.can_hold(&rect2));
    can_hold_print("rect1", "rect3", rect1.can_hold(&rect3));
    can_hold_print("rect2", "rect3", rect2.can_hold(&rect3));
}