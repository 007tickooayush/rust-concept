pub mod collections {
    use std::collections::HashMap;
    use unicode_segmentation::UnicodeSegmentation;

    pub fn test_collections() {
        let a = [1, 2, 3];

        let mut v = Vec::new();

        v.push(1);
        v.push(2);
        v.push(4);

        {
            // testing the heap and scope of v2
            // using vec macro
            let v2 = vec![1, 2, 2, 4];
            println!("the vector: {:#?}", v2);
        }

        let third = &v[2];
        // println!("Third element: {}",third);
        match v.get(2) {
            Some(third) => println!("Third element: {}", third),
            None => println!("There is no third element"),
        }

        // taking the mutable reference and mutating it while traversal
        print!("The vector: ");
        let mut vn = vec![11, 22, 33];
        for i in &mut vn {
            *i += 50;
            print!("{} ", i);
        }
        println!();
    }

    pub fn test_enum_multiple_type() {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("bkue")),
            SpreadsheetCell::Float(69.21),
        ];

        match &row[1] {
            SpreadsheetCell::Int(i) => {
                println!("{} is an integer", i);
            }
            _ => println!("Not an integer"),
        }
    }

    pub fn test_strings() {
        //
        let mut s1 = String::new();
        s1.push_str("ayush");

        // if the ownership is moved from s1 to any other variable
        // example
        let s3 = s1;

        let s4 = String::from("new") + &s3;

        // uncomment the line to confirm
        // println!("{}", s1);

        format!("{}{}", s3, s4);

        let custom_str = "नमस्ते";

        // here the custom_str[0] gives the first byte not the first char(Scalar)
        // println!("{}", custom_str);

        for i in "नमस्ते".bytes() {
            print!(":{}:", i)
        }
        println!();

        // iterate over scaler values:
        // ['न','म','स्','ते']

        // grapheme clusters (need "unicode-segmentation" crate for it)
        // ["न","म","स्","ते"]

        for g in "नमस्ते".graphemes(true) {
            // as here .chars() function does not work
            print!("{} ", g);
        }
        println!();

        // check the difference by uncommenting the below code
        // {
        //     for c in "नमस्ते".chars() {
        //         print!("{} ",c);
        //     }println!();
        // }
    }

    pub fn test_hashmap() {
        let blue = String::from("blue");
        let yellow = String::from("yellow");

        let mut scores = HashMap::new();

        scores.insert(blue, 10); // add line to specify types
        scores.insert(yellow, 20);

        // can not access without using lifetimes as ownership of "blue" variable is passed
        // println!(blue);

        // provided Option because the key may or may not be present
        let score = scores.get(&String::from("blue"));

        match score {
            Some(10) => {
                println!("stored score for team")
            }
            _ => println!("Specified team not found"),
        }

        // insert without overriding the key value

        // overriding ->
        scores.insert(String::from("black"), 10);
        scores.insert(String::from("black"), 100);

        println!("Map: {:#?}", scores);

        // without overriding ->
        scores.entry(String::from("gray")).or_insert(30); // inserts after checking
        scores.entry(String::from("gray")).or_insert(300); // does not insert

        println!("Map: {:#?}", scores);

        hashmap_task();
    }

    pub fn hashmap_task() {
        let text = "hello Rust is a programming language";

        let mut map = HashMap::new();

        // ["hello","Rust","a","programming","language","is"]
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("Count map: {:#?}", map)
    }

    pub fn test_all(){
        // test_rectangle();
        test_collections();
        test_enum_multiple_type();
        test_strings();
        test_hashmap();
    }
}
