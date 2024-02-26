pub mod interior_mutability_mem_safe {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::smart_pointers_interior_mem_safe::interior_mutability_mem_safe::List::{Cons, Nothing};

    #[derive(Debug)]
    pub enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nothing
    }
    impl List {
        pub fn tail(&self) -> Option<&RefCell<Rc<List>>>{
            match self {
                Cons(_,item) => Some(item),
                Nothing => None
            }
        }
    }
    pub fn test_stack_interior() {
        let a = Rc::new(Cons(10,RefCell::new(Rc::new(Nothing))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());

    }
}