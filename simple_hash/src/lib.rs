use std::collections::HashMap;


pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut word_hash_map: HashMap<&str, usize> = HashMap::new();
    for word in words {
        let count = word_hash_map.entry(word).or_insert(0);
        *count += 1;
        // word_hash_map.insert(word, 1);
    }
    word_hash_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut count = 0;
    for (_key, value) in frequency_count {
        if *value == 1 {
            count += 1;
        }
    }    
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let sentence = "this is a very basic sentence with only few \
                repetitions. once again this is very basic and \
                but it should be enough for basic tests".to_string();
    let words = sentence.split(" ").collect::<Vec<&str>>();

    let frequency_count = word_frequency_counter(words);
    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
    }
}
