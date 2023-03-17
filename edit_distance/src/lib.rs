pub fn edit_distance(source: &str, target: &str) -> usize {
    println!("{}, {}", source, target);

    let mut distance = 0;
    let mut working_target = target.to_string();
    for char in source.chars() {
        if let Some(index) = working_target.find(char) {
            working_target = working_target[..index].to_string() + &working_target[index+1..];
            // working_target = new_target.to_string();
        }  else {
            distance += 1;
        }
        println!("{}, {}", working_target, distance);
        if working_target.len() == 0 {
            break;
        }
    }
    let mut source_len = source.len();
    if source.len() > target.len() {
        source_len -= working_target.len();
    } else {

    }
    distance += source_len.abs_diff(target.len());
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let source = "alignment";
	let target = "assignment";
	println!(
		"It's necessary to make {} change(s) to {}, to get {}",
		edit_distance(source, target),
		source,
		target
	);
    }
}
