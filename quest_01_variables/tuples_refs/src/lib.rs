#[derive(Debug)]

pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    student.0
    
}

pub fn first_name(student: &Student) -> String {
    let std = &student.1;
    std.to_string()
}

pub fn last_name(student: &Student) -> String {
    let std = &student.2;
    std.to_string()
}