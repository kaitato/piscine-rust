use chrono::{NaiveDate, Utc};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut errors = vec![];

        if self.first_name.is_empty() {
            let err = FormError::new("first_name".to_string(), self.first_name.clone(), "No user name".to_string());
            return Err(err);
        } else {
            errors.push("Valid first name");
        }

        if self.password.len() < 8 {
            let err = FormError::new("password".to_string(), self.password.clone(), "At least 8 characters".to_string());
            return Err(err);
        }

        let mut has_alpha = false;
        let mut has_digit = false;
        let mut has_non_alphanum = false;

        for c in self.password.chars() {
            if c.is_alphabetic() {
                has_alpha = true;
            } else if c.is_digit(10) {
                has_digit = true;
            } else if !c.is_ascii_alphanumeric() {
                has_non_alphanum = true;
            }
        }

        if !has_alpha || !has_digit || !has_non_alphanum {
            let err = FormError::new("password".to_string(), self.password.clone(), "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string());
            return Err(err);
        } else {
            errors.push("Valid password");
        }

        Ok(errors)
    }
}

pub fn create_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
