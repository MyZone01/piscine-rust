use std::iter::Iterator;
use std::ops::{Add, AddAssign};

pub struct StepIterator<T> {
    end: T,
    step: T,
    current: T,
}

impl<T: PartialOrd + Add<Output = T>> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            end,
            step,
            current: beg,
        }
    }
}

impl<T: Copy + PartialOrd + Add<Output = T> + AddAssign> Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }

        let current = self.current;
        self.current += self.step;
        Some(current)
    }
}
