pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match server {
        Ok(url) => url,
        Err(_) => match security_level {
            Security::Unknown => server.unwrap(),
            Security::High => server.expect("ERROR: program stops"),
            Security::Medium => server.expect("WARNING: check the server"),
            Security::Low => server.expect("Not found: [SERVER_URL]"),
            Security::BlockServer => match server {
                Ok(a) => a,
                Err(_) => panic!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", fetch_data(Ok("server1.com".to_string()), Security::Medium));
        println!("{}", fetch_data(Err(String::new()), Security::Medium));
        println!("{}", fetch_data(Err("server2.com".to_string()), Security::Low));
    
        // Panics with no custom message
        // fetch_data(Err("ERROR CRITICAL".to_string()), Security::Unknown);
    
        // Panics with the message "ERROR: program stops"
        // fetch_data(Err(String::new()), Security::High);
    
        // Panics with the message "malicious_server.com"
        // fetch_data(Ok("malicious_server.com".to_string()), Security::BlockServer);
    }
}
