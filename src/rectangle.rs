mod rectangle_mod {
    use crate::enums::IPAddrKind;

    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        // all methods have '&self' parameter, and associated functions do not
        pub fn calculate_area(&self) -> u32 {
            self.width * self.height
        }

        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width || self.height > other.height
        }
    }

    impl Rectangle {
        pub fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    pub fn test_rectangle() {
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
        const IP: IPAddrKind = IPAddrKind::V4;
        println!("IP address type: {:#?}", IP);

        println!("{}", "-".repeat(100));
    }
}

// pub use rectangle_mod::*;
pub use self::rectangle_mod::*;
