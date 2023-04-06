pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return arr;
    }

    let mut products = vec![1; arr.len()];

    let mut product_so_far = 1;
    for i in 0..arr.len() {
        products[i] *= product_so_far;
        product_so_far *= arr[i];
    }

    product_so_far = 1;
    for i in (0..arr.len()).rev() {
        products[i] *= product_so_far;
        product_so_far *= arr[i];
    }

    products
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
