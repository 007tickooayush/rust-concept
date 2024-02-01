#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
fn main() {
    let rect = Rectangle {
        width: 500,
        height: 20
    };

    // printing the Struct using the prettifying pattern
    println!("{}","-".repeat(100));
    println!("{:#?}",rect);
    println!("{}","=".repeat(100));
    println!("calculated area: {}",calculate_area(&rect));
    println!("{}","-".repeat(100));
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

