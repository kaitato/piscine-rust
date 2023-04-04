// level 1: inv_pyramid (karyun.cheung)

pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
	let mut res = Vec::new();
	for j in 0..i {
		let mut line = String::new();
		for _ in 0..=j {
			line.push_str(" ");
		}
		for _ in 0..=j {
			line.push_str(&v);
		}
		res.push(line);
	}
	if i <= 1 { return res }
	for k in (0..i-1).rev() {
		let mut line = String::new();
		for _ in 0..=k {
			line.push_str(" ");
		}
		for _ in 0..=k {
			line.push_str(&v);
		}
		res.push(line);
	}
	res
}