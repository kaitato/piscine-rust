// use core::panicking::panic;
use std::fs::File;
use std::io::{ErrorKind, Write};
// use std::str::Bytes;

pub fn open_or_create(file: &str, content: &str) {
    let file_result = File::open(file);
    let _open_file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(mut fc) => {
                    fc.write(content.as_bytes()).unwrap();
                    fc
                
                },
                Err(e) => panic!("Problem Creating File: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}",other_error);
            }
        },
    };

}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     let path = "a.txt";
//     File::create(path).unwrap();
//     open_or_create(path, "content to be written");

//     let mut file = File::open(path).unwrap();

//     let mut s = String::new();
//     file.read_to_string(&mut s).unwrap();
//     println!("{}", s);
//     }
// }
