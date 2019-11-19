#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    let a = area(&rect1);
    println!("The area of rect1 is {}", a);

    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
