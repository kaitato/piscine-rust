pub use case;
pub use case::CaseExt;
pub use edit_distance::edit_distance;



pub fn expected_variable(string_to_compare: &str, expected_string: &str) -> Option<String> {
    let string_lowercase = &string_to_compare.to_ascii_lowercase();
    
    if (string_lowercase == &string_to_compare.to_lowercase() || &string_lowercase.to_camel_lowercase() == string_lowercase) && !string_lowercase.contains('-') && !string_lowercase.contains(' '){
        let distance  = edit_distance(&string_lowercase, &expected_string.to_lowercase());
        println!("{}", distance);
        if distance == 0 {
            return Some("100%".to_string())
        }
        let mut alikeness = 100;
        if distance * 100 / expected_string.len() > 100 {
            alikeness = 0;
        } else {
            alikeness = 100 - (distance * 100 / expected_string.len());

        }
        println!("{}, {}, {}, {}", alikeness, distance, expected_string, string_to_compare);
        println!("{}", Some(alikeness.to_string() + "%").unwrap());
        if alikeness >= 50 {
            Some(alikeness.to_string() + "%")
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{} close to it",
            expected_variable("On_Point", "on_point").unwrap()
        );
        println!(
            "{} close to it",
            expected_variable("soClose", "So_Close").unwrap()
        );
        println!(
            "{:?}",
            expected_variable("something", "something_completely_different")
        );
        println!(
            "{} close to it",
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        );
    }
}