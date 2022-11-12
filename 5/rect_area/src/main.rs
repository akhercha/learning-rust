// We use this line below to be able to print our struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect: Rectangle = dbg!(Rectangle {
        width: 30,
        height: 50,
    });

    dbg!(&rect); // here does not take ownership like up

    println!(
        "Its area is {} square pixels.",
        area(&rect)
    );
}
