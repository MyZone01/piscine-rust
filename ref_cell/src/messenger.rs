use std::{cell::RefCell, rc::Rc};

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    pub value: RefCell<usize>,
    max: u32,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: u32) -> Self {
        Tracker {
            logger,
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        *self.value.borrow_mut() = Rc::strong_count(value);
        let percentage = (*self.value.borrow() * 100) as u32 / self.max;
        if percentage >= 100 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            ));
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let percentage = (Rc::strong_count(value) * 100) as u32 / self.max;
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            percentage
        ));
        println!("💀 Info");
    }
}
