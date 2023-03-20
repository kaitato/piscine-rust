pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let source_len = source_chars.len();
    let target_len = target_chars.len();

    // Initialize the distance matrix with 0s
    let mut dist = vec![vec![0; target_len + 1]; source_len + 1];

    // Fill the first row and column with the distance from the empty string
    for i in 0..=source_len {
        dist[i][0] = i;
    }
    for j in 0..=target_len {
        dist[0][j] = j;
    }

    // Fill the rest of the matrix
    for i in 1..=source_len {
        for j in 1..=target_len {
            let mut cost = 1;
            if source_chars[i - 1] == target_chars[j - 1] {
                cost = 0;
            }
            dist[i][j] = usize::min(
                usize::min(dist[i - 1][j] + 1, dist[i][j - 1] + 1),
                dist[i - 1][j - 1] + cost,
            );
        }
    }

    // Return the minimum edit distance
    dist[source_len][target_len]
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
    let source = "gumbo";
	let target = "gambol";
	println!(
		"It's necessary to make {} change(s) to {}, to get {}",
		edit_distance(source, target),
		source,
		target
	);
    let source = "kitten";
	let target = "sitting";
	println!(
		"It's necessary to make {} change(s) to {}, to get {}",
		edit_distance(source, target),
		source,
		target
	);
    let source = "rosettacode";
	let target = "raisethysword";
	println!(
		"It's necessary to make {} change(s) to {}, to get {}",
		edit_distance(source, target),
		source,
		target
	);
    }
}
