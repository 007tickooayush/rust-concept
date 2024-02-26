pub mod smart_pointers_refCell {
    use std::cell::RefCell;

    use self::refcell_example::{LimitTracker, Messenger, MessengerObj};

    pub fn test_without_ref(){
        let a = 5;
        // let b = &mut a; // will give error

        let mut c = 10;
        let d = &c;
        // *d = 20; // will give error
    }
    pub fn test_with_ref() {
        let messenger = MessengerObj::new();
        
        // let messenger = Messenger::new();
        let mut limit_tracker = LimitTracker::new(&messenger, 100);
        limit_tracker.set_value(80);

        // assert_eq!(messenger.sent_messages.len(),1); // gives error cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
        
        // SOLVED USING INTERIOR MUTABILITY
        assert_eq!(messenger.sent_messages.borrow().len(), 1);

    }

    pub mod refcell_example {
        use std::cell::RefCell;


        // implemening the Messenger for a struct
        pub struct MessengerObj {
            pub sent_messages: RefCell<Vec<String>>,
        }

        impl MessengerObj {
            pub fn new() -> MessengerObj {
                MessengerObj {
                    // sent_messages: vec![], // gives error
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        // IMPLEMENT THE MESSENGER TRAIT FINALLY
        impl Messenger for MessengerObj {
            fn send(&self, msg: &str) {
                // self.send(msg);
                self.sent_messages.borrow_mut().push(String::from(msg));
            }
        }

        pub trait Messenger {
            fn send(&self, msg: &str);
        }
        pub struct LimitTracker<'a, T:Messenger> {
            messenger: &'a T,
            value: usize,
            max: usize,
        }


        impl <'a, T> LimitTracker<'a, T> where T: Messenger {
            pub fn new(messenger:&'a T, max: usize) -> LimitTracker<'a, T> {
                LimitTracker {
                    messenger,
                    value: 0,
                    max
                }
            }

            pub fn set_value(&mut self, value: usize) {
                self.value = value;

                let percentage_of_max = self.value as f64 / self.max as f64;

                if percentage_of_max >= 1.0 {
                    self.messenger.send("ERROR: Quota exceeded");
                } else if percentage_of_max >= 0.9 {
                    self.messenger.send("URGENT WARNING: 90% quota exceeded");
                } else if percentage_of_max >= 0.75 {
                    self.messenger.send("WARNING: 75% quota exceeded");
                }
            }
        }
    }

}