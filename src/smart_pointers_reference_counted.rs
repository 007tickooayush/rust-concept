pub mod smart_pointers_rc {
    use std::rc::Rc;


    // List with Rc<T>
    #[derive(Debug)]
    pub enum List {
        Cons(i32,Rc<List>),
        Nil
    }
    use self::List::{Cons,Nil};

    // List without Rc<T>
    #[derive(Debug)] // Implement Debug trait for ListWithoutRc
    pub enum ListWithoutRc {
        Cons(i32,Box<ListWithoutRc>),
        Nil
    }

    pub fn test_reference_counted () {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

        // Share the ownership of the reference counted value
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));

        println!("With Rc Reference Counted Pointer => {:?}", a);
        println!("With Rc Reference Counted Pointer => {:?}", b);
        println!("With Rc Reference Counted Pointer => {:?}", c);
    }

    pub fn test_without_rc () {
        use self::ListWithoutRc::{Cons,Nil};
        let a = Box::new(Cons(5, Box::new(Cons(10, Box::new(Nil)))));
        let b = Cons(3, Box::new(*a));

        // invalid syntax : borrow of moved value: `a`,move occurs because `*a` has type `ListWithoutRc`, which does not implement the `Copy` trait
        // println!("Without Reference Counted:>> {:?}", a); 
        // Causes error: value used here after move
        println!("Without Reference Counted:>> {:?}", b);
    }
    
}