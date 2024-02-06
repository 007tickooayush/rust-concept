pub mod lifetimes_mod {
    
    pub fn test_vars_lifetime() {
        
        // below r becomes the dangling reference due to the lifetime of x validated by the borrow checker
        // let r;
        // {
        //     let x = 5;
        //     r = &x;
        // }
        // println!("r: {}", r);

        // below r is valid as the lifetime of x is greater than r
        let s = 5;
        let r = &s;
        println!("r: {}", r);

        let string1 = String::from("abcd");
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);

        

    }

    // creates a lifetime ambiguity problem as it is not fixed where x is to be returned or y
    // pub fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // generic lifetime annotations
    // &i32       // a reference
    // &'a i32    // a reference with an explicit lifetime
    // &'a mut i32// a mutable reference with an explicit lifetime


    // below function solves the issue using generic lifetime annotations
    pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        return if a.len() > b.len() { a } else { b };
    }

}

use lifetimes_mod::*;