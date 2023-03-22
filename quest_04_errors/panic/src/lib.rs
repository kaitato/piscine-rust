use std::fs::File;
// use panic::*;

pub fn open_file(s: &str) -> File {
    let file = File::open(s).expect("File not found");
    file
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     let filename = "created.txt";
//     File::create(filename).unwrap();

//     let a = open_file(filename);
//     println!("{:?}", a);
    
//     fs::remove_file(filename).unwrap();

//     //It must panic
//     let b = open_file(filename);
//     }
// }
