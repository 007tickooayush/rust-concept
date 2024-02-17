
use closures::closures_mod::{generate_workout, test_move_keyword};
use collections::collections::test_collections;
use error_handling::{test_file_error, test_file_error_propagation};
use iterators::iterators::iterator_test;
use lifetimes::lifetimes_mod::test_vars_lifetime;
use smart_pointers::{test_box, test_box_multiple};
use traits::traits_mod::{test_traits,notify,notify_2};
use types::types_mod::{get_largest, test_types};

mod collections;
mod enums;
mod error_handling;
mod rectangle;
mod types;
mod traits;
mod lifetimes;
mod closures;
mod iterators;

pub mod smart_pointers {
    pub fn test_box() {
        let b = Box::new(201);

        println!("b = {}", b);
    }

    // testing Box implementation for List Enum with Dynamic length
    // Generic type implementation
    pub enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    pub fn test_box_enum(){
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        
    }


    #[derive(Debug)]
    pub enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    pub fn test_box_multiple(){
        let msg = Message::Write(String::from("hello"));
        let color = Message::ChangeColor(0, 0, 0);
        let quit = Message::Quit;
        let move_msg = Message::Move { x: 0, y: 0 };

        println!("msg = {:?}", msg);
        println!("color = {:?}", color);
        println!("quit = {:?}", quit);
        println!("move_msg = {:?}", move_msg);
    }


    // primitive type implementation
    // pub enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }



    // testing Box implementation for Drop trait

}


fn main() {

    test_box();
    test_box_multiple();
    
    // iterator_test();
    // generate_workout(7, 2);
    // test_move_keyword();
    // test_vars_lifetime();
    // test_traits();
    // test_types();
    // test_rectangle();
    // test_collections();

    // let _ = test_file_error_propagation();
    // test_file_error();
}
