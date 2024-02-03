
pub mod types_mod {

    pub struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    #[derive(Debug)]
    pub enum Option<T> {
        Some(T),
        None,
    }

    pub fn test_types() {
        let number_list = vec![34, 50, 25, 100, 65];

        let largest = get_largest(number_list);

        println!("The largest number is {}", largest);

        let char_list = vec!['y', 'm', 'a', 'q'];

        println!("The largest char is {}", get_largest(char_list));

        // testing generic type structs
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: 1.0, y: 4.0 };
        let p3 = Point { x: 5, y: 4.0 };

        println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
        println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

        let p4 = p1.mixup(p2);
        println!("p4.x = {}, p4.y = {}", p4.x, p4.y);

        // testing Option enum
        let some_number = Option::Some(5);
        let other_number = Option::Some(6.001);
        let some_string = Option::Some("a string");

        let absent_number: Option<i32> = Option::None;

        println!(
            "some_number: {:?}, other_number: {:?},  some_string: {:?}, absent_number: {:?} ",
            some_number, other_number, some_string, absent_number
        );
    }

    // making the function generic
    pub fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
        let mut largest = list[0];

        for num in list {
            if num > largest {
                largest = num;
            }
        }
        largest
    }
}

use self::types_mod::*;