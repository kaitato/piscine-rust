use chrono::{Datelike, NaiveDate, Duration};
pub use chrono::Weekday as wd;

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    let days_in_year = if is_leap_year { 366 } else { 365 };
    if days_in_year % 2 == 0 {
        return None;
    }
    let middle_day_num = (days_in_year / 2) + 1;
    let middle_date = (NaiveDate::from_ymd(year, 1, 1) + Duration::days(middle_day_num))
        .weekday();
    Some(middle_date)

}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let year = 2023;
//         let jan_1 = NaiveDate::from_ymd(year, 1, 1);
//         let middle_day = jan_1 + chrono::Duration::days(182);
//         println!("Middle day of the year {} is {}", year, middle_day);
//     }
// }
