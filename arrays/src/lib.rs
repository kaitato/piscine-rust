pub fn sum(a: &[i32]) -> i32 {
    let result;
    for i in a {
        result += i;
    }
    result
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
// 	let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// 	let a1: Vec<i32> = (1..11).; //missing info here
// 	let b = [_; 10]; //missing info here

// 	println!("The Sum of the elements in {:?} = {}", a, sum(a));//missing info here
// 	println!("The Sum of the elements in {:?} = ", a1, sum(a1));//missing info here
// 	println!("The Sum of the elements in {:?} = {}", b, sum(b));//missing info here
// 	println!(
// 		"Array size {} with only 10's in it {:?}",
// 		thirtytwo_tens().len(),
// 		thirtytwo_tens()
// 	);
//     }
// }
