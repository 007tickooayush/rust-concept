pub mod traits_mod {
    use std::fmt::Display;

    pub struct NewsArticle {
        pub headline: String,
        pub author: String,
        pub content: String,
    }

    impl Summarization for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.content)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarization for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub trait Summarization {
        fn summarize(&self) -> String; // does not have a default implementation and needs to be implemented
        fn summarize_author(&self) -> String {
            // have a default implementation
            String::from("Read more...")
        }
    }

    // traits as parameters
    // pub fn notify(item: &impl Summarization) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // without the syntax sugar
    // here the types are restricted and only one type can be passed
    pub fn notify<T: Summarization>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify_2<T: Summarization>(item1: &T, item2: &T) {
        // do something
        println!("1. Breaking news! {}", item1.summarize());
        println!("2. Breaking news! {}", item2.summarize());
    }


    // for readability, we can use the where clause
    pub fn notify_where<T,U> (t : &T, u: &U) -> i32
    where T: Summarization + PartialOrd + Display,
          U: Summarization + Clone + Display {
        return 0;
    }

    pub fn test_traits(){
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
    
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };
    
        println!("1 new tweet: {}", tweet.summarize());
        println!("New article available! {}", article.summarize());
        println!("Article Author: {}", article.summarize_author());
    }

    pub mod advanced_traits {
        use std::ops::Add;

        /// CREATE A DEFINITION FOR TWO DIFERENT UNITS OF METRIC SYSTEM
        /// AND PROVIDE FUNCTIONS TO MANIPULATE THEM
        #[derive(Debug)]
        struct Millimeters(u32);
        #[derive(Debug)]
        struct Meters(u32);

        /// IMPLEMENTATION FOR ADDING DIFFERENT UNITS
        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            /// RETURNS THE METERS ADDED INTO THE MILLIMETER UNIT
            fn add(self, rhs: Meters) -> Self::Output {
                Millimeters(self.0 + rhs.0 * 1000)
            }
        }

        // UNAMBIGUITY OF ASSOCIATED FUNCTIONS
        pub trait Pilot {
            fn fly(&self);
        }

        pub trait Wizard {
            fn fly(&self);
        }

        #[derive(Debug)]
        pub struct Human(String);
        impl Human {
            /// 
            /// 
            /// # Arguments 
            /// 
            /// * `name`: 
            /// 
            /// returns: Human 
            /// 
            /// # Examples 
            /// 
            /// ```
            /// 
            /// ```
            pub fn new(name: &str) -> Human {
                Human(String::from(name))
            }
            
            pub fn fly(&self) {
                println!("{} is flying", &self.0);
            }
        }

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking '{}'",&self.0)
            }
        }
        
        impl Wizard for Human {
            fn fly(&self) {
                println!("Up we go by the soaring feat of '{}'",&self.0)
            }
        }
        
        
        #[test]
        pub fn test_traits_single(){
            let person = Human::new("John");
            person.fly();
        }
        
        #[test]
        pub fn test_traits_multiple(){
            let person = Human::new("John");
            Pilot::fly(&person);
            Wizard::fly(&person);
        }
        
        #[test]
        pub fn test_millimeters(){
            let mm = Millimeters(1000);
            let m = Meters(1);

            let result = mm + m;
            // println!("Result: {:?}", result);
            assert_eq!(result.0, 2000);
        }

    }
}


use self::traits_mod::*;