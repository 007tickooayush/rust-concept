mod rectangle;
mod enums;

use crate::{enums::IPAddrKind, rectangle::Rectangle};


fn main() {
    let rect = Rectangle {
        width: 500,
        height: 20,
    };

    let rect1 = Rectangle {
        width: 100,
        height: 25,
    };

    let rect2 = Rectangle {
        width: 600,
        height: 20,
    };

    let rect3 = Rectangle::square(32); // using associated function of struct Rectangle

    // printing the Struct using the prettifying pattern
    println!("{}", "-".repeat(100));
    println!("{:#?}", rect);
    
    println!("{}", "=".repeat(100));
    println!("calculated area: {}", rect.calculate_area());

    println!("{}", "=".repeat(100));
    println!("rect can_hold rect1: {}", rect.can_hold(&rect1)); // passing the reference to not provide the ownership to the function

    println!("{}", "=".repeat(100));
    println!("rect can_hold rect2: {}", rect.can_hold(&rect2));

    println!("{}", "=".repeat(100));
    println!("Square rect3: {:#?}", rect3);

    println!("{}", "=".repeat(100));
    const ip:IPAddrKind = IPAddrKind::V4;
    println!("IP address type: {:#?}",ip);
    
    println!("{}", "-".repeat(100));
    
}
