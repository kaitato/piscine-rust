// use crate::edit_distance;
pub use case;
pub use case::CaseExt;
pub use edit_distance::edit_distance;
// pub mod edit_distance;
// pub use crate::edit_distance::edit_distance;
// pub use edit_distance::edit_distance;


pub fn expected_variable(string_to_compare: &str, expected_string: &str) -> Option<String> {
    if string_to_compare.is_camel_lowercase() == false && string_to_compare.contains('_') == false {
        return None
    } else if string_to_compare.is_empty() || expected_string.is_empty() {
            return None
        
    } else {
        let distance  = edit_distance(&string_to_compare.to_lowercase(), &expected_string.to_lowercase());
        let mut alikeness = 0;
        if (distance * 100 / std::cmp::max(string_to_compare.len(), expected_string.len())) > 100{
            alikeness = 0;        
        } else {
            alikeness = 100 - (distance * 100 / std::cmp::max(string_to_compare.len(), expected_string.len()));

        }
        // println!("{}, {}, {}, {}", alikeness, distance, expected_string.len(), string_to_compare.len());
        // println!("{}", Some(alikeness.to_string() + "%").unwrap());
        match alikeness {
            a if a > 50 => Some(alikeness.to_string() + "%"),
            _ => None,
            }
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