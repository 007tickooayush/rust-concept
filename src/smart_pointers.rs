
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