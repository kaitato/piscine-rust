// use crate::Worker;
use std::{cell::RefCell, rc::Rc};



pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}
pub struct Tracker<'a, T: Logger> {
    pub logger: &'a T,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Tracker<T> {
        Tracker {
            logger,
            value: RefCell::new(0),
            max,
        }
    }
    pub fn set_value(&self, value: &Rc<usize>) {
        *self.value.borrow_mut() = Rc::strong_count(&value);
        let percent = *self.value.borrow() * 100 / self.max;
        if percent >= 100 {
            self.logger.error("Error: you are over your quota!");
        } else if percent >= 70 {
            self.logger.warning(&format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", percent));
        }
    }
    pub fn peek(&self, track_value : &Rc<usize>) {
        let percent = Rc::strong_count(&track_value)  * 100 / self.max;
        self.logger.info(&format!("Info: you are using up to {}% of your quota", percent));
    }
}