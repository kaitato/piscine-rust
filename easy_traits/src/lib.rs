#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;
    fn append_number(&mut self, nb_to_append: f64) -> Self;
    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        self.clone()
    }
    fn append_number(&mut self, nb_to_append: f64) -> Self {
        self.value.push_str(&nb_to_append.to_string());
        self.clone()
    }
    fn remove_punctuation_marks(&mut self) -> Self {
        self.value.retain(|c| !matches!(c, '.' | ',' | '?' | '!'));
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_str() {
        let mut str_aux = StringValue {
            value: String::from("hello"),
        };

        assert_eq!(
            String::from("hello there!"),
            str_aux.append_str(String::from(" there!")).value
        );

        assert_eq!(
            String::from("hello there! How are You?"),
            str_aux.append_str(String::from(" How are You?")).value
        );

        assert_eq!(
            String::from("hello there How are You"),
            str_aux.remove_punctuation_marks().value
        );
    }

    #[test]
    fn test_remove_punctuation() {
        let mut str_aux = StringValue {
            value: String::from("!?.,!?.,"),
        };

        assert_eq!(String::from(""), str_aux.remove_punctuation_marks().value);

        assert_eq!(
            String::from("h!e!l?lo. the,.re!"),
            str_aux.append_str(String::from("h!e!l?lo. the,.re!")).value
        );
        assert_eq!(
            String::from("hello there"),
            str_aux.remove_punctuation_marks().value
        );
    }

    #[test]
    fn test_append_number() {
        let mut str_aux = StringValue {
            value: String::from(""),
        };

        assert_eq!(String::from("-1"), str_aux.append_number(-1.0).value);

        assert_eq!(String::from("-15"), str_aux.append_number(5.0).value);

        assert_eq!(String::from("-155.5"), str_aux.append_number(5.5).value);

        assert_eq!(
            String::from("-1555"),
            str_aux.remove_punctuation_marks().value
        );
    }
}