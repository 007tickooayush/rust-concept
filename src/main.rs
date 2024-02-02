use core::panic;
use std::{fs::{self, File}, io::{self, ErrorKind, Read}};

mod rectangle;
mod enums;
mod collections;


pub fn test_error(){
    
}

pub fn test_int_error(){
    let i = 22;

    if i == 22 {
        panic!("Do not pass 22");
    }
}

pub fn test_file_error(){
    
    // The error of Result<T,E> type
    let f = File::open("hello.txt");


    // simplest way of handling the error
    // let f = File::open("hello.txt").expect("Failed to open the file");

    // ungraceful handling
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("File ERROR: {:#?}",error)
    // };

    // handling the errors gracefully (but code written without closures)
    // {
    //     let f = match f {
    //         Ok(file) => file,
    //         Err(error) => match error.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("problem creating file: {:#?}",e)
    //             },
    //             all_other_error_kind => {
    //                 panic!("Other Error occured while opening: {:#?}",all_other_error_kind)
    //             }
    //         }
    //     };
    // }

    // handling the error with closure
    {
        let f = File::open("hello.txt").unwrap_or_else( |err | {
            if err.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|err | {
                    panic!("Problem creating file: {:#?}",err)
                })
            }else {
                panic!("Problem opening file: {:#?}",err)
            }
        });
    }
    
}

pub fn test_file_error_propagation() -> Result<String, io::Error>{
    // adding ? which is similar functionality as unwrap_or_else() function
    // {
    //     let mut f = File::open("hello.txt")?;

    //     let mut s = String::new();

    //     f.read_to_string(&mut s)?;

    //     // return the result if the file is present
    //     Ok(s)
    // }

    // shortest format of code which returns the specified type
    fs::read_to_string("hello.txt")
    

    // but the ? operator can not be used in main function without customizing the main function by using Unit and Boxing the type

}

fn main() {
    test_file_error();
}
