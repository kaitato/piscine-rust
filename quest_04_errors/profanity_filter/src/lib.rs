pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.contains("stupid") || self.content == "" {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(_) => (true, &ms.content),
        None => (false, "ERROR: illegal"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
  let m0 = Message::new("hello there".to_string(), "toby".to_string());
  println!("{:?}", check_ms(&m0));

  let m1 = Message::new("".to_string(), "toby".to_string());
  println!("{:?}", check_ms(&m1));

  let m2 = Message::new("you are stupid".to_string(), "toby".to_string());
  println!("{:?}", check_ms(&m2));

  let m3 = Message::new("stupid".to_string(), "toby".to_string());
  println!("{:?}", check_ms(&m3));
    }
}
