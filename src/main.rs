use core::panic;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {

    //unrecoverable errors
    //using the panic macro
   // panic!("some fatal error occurred!"); //causes program to unwind the program

    let numbers = vec![3,6];
    // println!("{}",numbers[8]);

    simple_file_opener();



}

    //recoverable errors:
    //A Result enum is used and contains generic Result<T, E>.
    //T is the type that will be returned in a succcessfull case within Ok
    //E is the Error type to be returned in the case of Failure

    //simple function to open an existing file. It fails if the function does not exist
    pub fn simple_file_opener() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file, 
            Err(error) => panic!("Could not open the file: {:?}", error),
        };

        //do something with this file buffer
        let mut buffer = String::new();
        // greeting_file.read_to_string(& mut buffer)?;
        Ok((buffer));
    }
