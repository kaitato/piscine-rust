use std::num::ParseFloatError;
use std::collections::HashMap;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        let mut lh = String::from("--");
        lh.push_str(l_h);
        let sh = String::from("-");
        sh.to_string().push(l_h.chars().next().unwrap());
        Flag {
            short_hand: sh,
            long_hand: lh,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags.get(&flag) {
            Some(callback) => {
                match callback(argv[0], argv[1]) {
                    Ok(result) => result,
                    Err(e) => format!("Error: {}", e),
                }
            }
            None => "Error: Flag not found".to_owned(),
        }
    }
}



pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_float = a.parse::<f64>()?;
    let b_float = b.parse::<f64>()?;
    Ok((a_float / b_float).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_float = a.parse::<f64>()?;
    let b_float = b.parse::<f64>()?;
    Ok((a_float % b_float).to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let mut handler = FlagsHandler { flags: HashMap::new() };

    let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
    let r = Flag::opt_flag(
        "remainder",
        "remainder of the division between two values, formula (a % b)",
    );

    handler.add_flag((d.short_hand, d.long_hand), div);
    handler.add_flag((r.short_hand, r.long_hand), rem);

    println!("{:?}", handler.exec_func(("-d".to_string(), "--division".to_string()), &["1.0", "2.0"]));

    println!("{:?}",handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "2.0"]));

    println!("{:?}",handler.exec_func(("-d".to_string(), "--division".to_string()), &["a", "2.0"]));

    println!("{:?}",handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "fd"]));
    }
}
