use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
        let len = list.len();
    let mid = len / 2;
    let sorted = &mut list.clone();
    sorted.sort();
    if len % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0 as i32
    } else {
        sorted[mid]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut count_map = HashMap::new();
    for &val in list {
        let count = count_map.entry(val).or_insert(0);
        *count += 1;
    }
    let mut mode = list[0];
    let mut max_count = 0;
    for (&val, &count) in &count_map {
        if count > max_count {
            mode = val;
            max_count = count;
        }
    }
    mode
}
