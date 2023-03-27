#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        self.grade = Some(Box::new(Worker {
            role: role,
            name: name,
            next: self.grade.take(),
        }));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        let last_worker = self.grade.clone();
        match last_worker {
            None => None,
            Some(w) => {
                let last_worker_name = w.name;
                self.grade = w.next;
                Some(last_worker_name)
            },
        }
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        let last_worker = self.grade.clone();
        match last_worker {
            None => None,
            Some(w) => Some((w.name.clone(), w.role.clone())),
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let mut list = WorkEnvironment::new();
    list.add_worker(String::from("CEO"), String::from("Marie"));
    list.add_worker(String::from("Manager"), String::from("Monica"));
    list.add_worker(String::from("Normal Worker"), String::from("Ana"));
    list.add_worker(String::from("Normal Worker"), String::from("Alice"));
    println!("{:#?}", list);

    println!("{:?}", list.last_worker());

    list.remove_worker();
    list.remove_worker();
    list.remove_worker();
    println!("{:?}", list);
    list.remove_worker();
    println!("{:?}", list);
    }
}
