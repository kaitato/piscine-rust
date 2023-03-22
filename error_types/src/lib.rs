use chrono::{NaiveDateTime, Utc};

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
    pub birth: NaiveDateTime,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDateTime,
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

    pub fn validate(&self) -> Result<(), FormError> {
        let mut errors: Vec<String> = vec![];

        // Validate first name
        if self.first_name.is_empty() {
            errors.push("No user name".to_string());
        }

        // Validate password
        let mut has_alphabetic = false;
        let mut has_numeric = false;
        let mut has_non_alphanumeric = false;
        for c in self.password.chars() {
            if c.is_alphabetic() {
                has_alphabetic = true;
            } else if c.is_numeric() {
                has_numeric = true;
            } else if !c.is_ascii_alphanumeric() {
                has_non_alphanumeric = true;
            }

            if has_alphabetic && has_numeric && has_non_alphanumeric {
                break;
            }
        }
        if self.password.len() < 8 {
            errors.push("At least 8 characters".to_string());
        } else if !(has_alphabetic && has_numeric && has_non_alphanumeric) {
            errors.push(
                "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string(),
            );
        }

        if !errors.is_empty() {
            let err = errors.join("; ");
            return Err(FormError::new("password".to_string(), self.password.clone(), err));
        }

        Ok(())
    }
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
