pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut vec: Vec<u32> = Vec::new();
    let split_string = s.split_whitespace();
    for nums in split_string {
        if nums.ends_with('k') {
            let mut num = nums[..nums.len() - 1].parse::<f64>().unwrap();
                num *= 1000.;
                vec.push(num as u32)
        } else {
            vec.push(nums.parse::<u32>().unwrap())
        }
    }
    return Box::new(vec)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let new_str = String::from("5.5k 8.9k 32");

        // creating a variable and we save it in the Heap
        let a_h = transform_and_save_on_heap(new_str);
        println!("Box value : {:?}", &a_h);
        println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_h)));
    
        let a_b_v = take_value_ownership(a_h);
        println!("value : {:?}", &a_b_v);
        println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_b_v)));
        // whenever the box, in this case "a_h", goes out of scope it will be deallocated, freed
    }
}
