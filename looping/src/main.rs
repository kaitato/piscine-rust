use std::io;

fn main() {
    let secret_answer: String = String::from("The letter e");
    let mut num_of_tries = 0;
    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut input = String::new();
    
        io::stdin().read_line(&mut input).unwrap();
    
        if input.trim().eq(&secret_answer) == true {
            println!("Number of trials: {}", num_of_tries);
            break;
        } else {
            num_of_tries = num_of_tries + 1
        }
    }
}
