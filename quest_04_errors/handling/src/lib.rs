use std::fs::File;
use std::fs;

pub fn open_or_create(file: &str, content: &str) {
    let file_result = File::open(file);

    let _f = match file_result {
        Ok(file) => file,
        Err(_) => match File::create(file) {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
    };
    fs::write(file, content).expect("Unable to write file");
    // let mut output = File::open(file);
    // write!(output, "{:?}", content)
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
