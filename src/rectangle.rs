mod rectangle_mod{
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
    
}

// pub use rectangle_mod::*;
pub use self::rectangle_mod::*;
