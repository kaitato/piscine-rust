use std::iter;

pub fn get_diamond(c: char) -> Vec<String> {
    let diamond_size = ((c as u8) - b'A') as usize * 2 + 1;

    let top_half: Vec<_> = iter::once(format!("{0:^1$}", 'A', diamond_size))
        .chain((1..=diamond_size / 2).map(|i| {
            format!(
                "{0:^1$}",
                format!("{0}{1}{0}", (b'A' + i as u8) as char, " ".repeat(i * 2 - 1)),
                diamond_size
            )
        }))
        .collect();

    let bottom_half:Vec<String> = top_half.iter()
        .rev()
        .skip(1)
        .cloned()
        .collect();

    top_half.iter()
        .chain(bottom_half.iter())
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
