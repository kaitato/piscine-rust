pub fn edit_distance(source: &str, target: &str) -> usize {
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
    if source.len() > target.len() {
        distance += working_target.len()
    }
    distance += (source.len()-working_target.len()).abs_diff(target.len());
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let target = "alignment";
	let source = "assignment";
	println!(
		"It's necessary to make {} change(s) to {}, to get {}",
		edit_distance(source, target),
		source,
		target
	);
    }
}
