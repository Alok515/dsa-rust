struct Rectangle {
    width: u32,
    height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn print_area( area: &u32) {
    println!("The area of the rectangle is {} square pixels.", area);
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect1_area = area(&rect1);
    print_area(&rect1_area);
}