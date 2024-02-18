// testing implementation for Deref Trait
pub mod smart_pointers_deref {
    pub trait Deref {
        type Target;
        fn deref(&self) -> &Self::Target;
    }


    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T { // here &Self::Target = &T
            &self.0
        }
    }

    // adding the Deref trait to MyBox

    pub fn demo_deref() {
        let x = 5;
        // let y = &x;

        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); // to  dereference the box we need the Deref Trait
    
    }

    pub fn test_deref_mybox() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        // for our implementation the deref is not automatically called
        assert_eq!(5, *(y.deref())); // to  dereference the box we need the Deref Trait
    }


}

