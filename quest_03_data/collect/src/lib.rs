pub fn bubble_sort(vec: &mut Vec<i32>) {
    let len = vec.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let ref mut v = vec![3, 2, 4, 5, 1, 7];
	let mut b = v.clone();
	bubble_sort(v);
	println!("{:?}", v);

	b.sort();
	println!("{:?}", b);
    }
}
