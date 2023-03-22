// // use crate::edit_distance;
// pub use case;
// pub use case::CaseExt;
// pub use edit_distance::edit_distance;
// // pub mod edit_distance;
// // pub use crate::edit_distance::edit_distance;
// // pub use edit_distance::edit_distance;


// pub fn expected_variable(string_to_compare: &str, expected_string: &str) -> Option<String> {
//     if string_to_compare.is_camel_lowercase() == false && string_to_compare.contains('_') == false {
//         return None
//     } else if string_to_compare.is_empty() || expected_string.is_empty() {
//             return None
        
//     } else {
//         let distance  = edit_distance(&string_to_compare.to_lowercase(), &expected_string.to_lowercase());
//         let mut alikeness = 0;
//         if (distance * 100 / std::cmp::max(string_to_compare.len(), expected_string.len())) > 100{
//             alikeness = 0;        
//         } else {
//             alikeness = 100 - (distance * 100 / std::cmp::max(string_to_compare.len(), expected_string.len()));

//         }
//         // println!("{}, {}, {}, {}", alikeness, distance, expected_string.len(), string_to_compare.len());
//         // println!("{}", Some(alikeness.to_string() + "%").unwrap());
//         if alikeness > 50 {
//             Some(alikeness.to_string() + "%")
//         } else {
//             None
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         println!(
//             "{} close to it",
//             expected_variable("On_Point", "on_point").unwrap()
//         );
//         println!(
//             "{} close to it",
//             expected_variable("soClose", "So_Close").unwrap()
//         );
//         println!(
//             "{:?}",
//             expected_variable("something", "something_completely_different")
//         );
//         println!(
//             "{} close to it",
//             expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
//         );
//     }
// }
pub use edit_distance::edit_distance;
pub fn expected_variable(s1: &str, s2: &str) -> Option<String> {
    let is_camel = s1.chars().next().map_or(false, char::is_uppercase) && s1.chars().any(|c| c == '_');
    let is_snake = s1.chars().all(|c| c == '_' || c.is_ascii_lowercase());

    if !is_camel && !is_snake {
        return None;
    }

    let distance = edit_distance(s1, s2);
    let expected_len = s2.len();
    let similarity_percentage = (expected_len - distance) * 100 / expected_len;

    if similarity_percentage > 50 {
        Some(format!("{}%", similarity_percentage))
    } else {
        None
    }
}