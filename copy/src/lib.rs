pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let tup = (c, (c as f64).exp(), (c.abs() as f64).ln());
    tup
}

pub fn str_function(a: String) -> (String, String) {
    let res = a.split(" ");
    let mut b = String::new();
    for num in res {
        let exp = num.parse::<f64>().unwrap().exp();
        b.push_str(&exp.to_string());
        b.push(' ');
    }
    b.truncate(b.len()-1);
    let tup = (a, b);
    tup
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut v:Vec<f64> = Vec::new();
    for a in &b {
        let a_f64 = (*a as f64).ln();
        v.push(a_f64)
    }
    let tup = (b, v);
    tup
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: i32 = -12;
        let b = String::from("1 2 4 5 6");
        let c = vec![1, 2, 4, 5];
    
        println!("{:?}", nbr_function(a));
        println!("{:?}", str_function(b));
        println!("{:?}", vec_function(c));
    }
}
