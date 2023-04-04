fn main() {
    println!("New worker: {:?}",
        OfficeWorker::from("Manuel,23,admin"));
    println!("New worker: {:?}",
        OfficeWorker::from("Jean Jacques,44,guest"));
}

// level 4: office_worker (karyun.cheung)

#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
	pub name: String,
	pub age: u32,
	pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
	Admin,
	User,
	Guest,
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
		let field: Vec<&str> = s.split(",").collect();
		OfficeWorker {
            name: field[0].to_string(),
            age: field[1].parse::<u32>().unwrap(),
            role: WorkerRole::from(field[2]),
        }
	}
}

impl From<&str> for WorkerRole {
	fn from(s: &str) -> Self {
		match s {
			"admin" => WorkerRole::Admin,
			"user" => WorkerRole::User,
			_ => WorkerRole::Guest,
		}
	}
}