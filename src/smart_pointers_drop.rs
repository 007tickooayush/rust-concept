pub mod smart_pointers_drop {
    pub struct SomeValue
    {
        data: String
    }

    impl Drop for SomeValue {
        fn drop(&mut self) {
            println!("Dropping SomeValue with data `{}`!", self.data);
        }
    }

    pub fn test_drop() {
        let x = SomeValue { data: String::from("some data from x variable") };
        println!("Made a SomeValue!");
        // x.drop(); // this is not allowed
        // drop(x); // this is allowed
        println!("Exiting the block!");

        let y = SomeValue { data: String::from("some data from y variable") }; // by default dropped first
        println!("Made a SomeValue!");

        // dropping in our own defind order
        // drop(x);
        // drop(y);
    }
}