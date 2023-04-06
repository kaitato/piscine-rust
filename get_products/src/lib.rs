pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut new_vec : Vec<usize> = Vec::new();
    if arr.len() <= 1{
        return vec![]
    }
    for (i,nums) in arr.iter().enumerate(){
        let mut index = 0;
        let mut sum = 1;
        if arr[i].clone() == *nums{
            // println!("{nums}");
            while index < arr.len(){
                if index != i{
                    sum= (sum) * arr[index];
                    print!(" {:} multiplied\n", arr[index]);
                    print!(" {sum} this is total.");
                }
                index += 1
            }
            println!("\nadded.");
            new_vec.push(sum);
        }
    };
    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: Vec<usize> = vec![1, 7, 3, 4];
        let output = get_products(arr);
        println!("{:?}", output);
    }
}
