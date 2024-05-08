use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let pid = self.track_worker();
        (pid, Thread::new_thread(pid, c, self))
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        let states = self.states.borrow();
        *states.get(id).unwrap_or(&false)
    }
    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if *states.get(id).unwrap_or(&false) {
            panic!("{} is already dropped", id);
        } else {
            *states.get_mut(id).unwrap() = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        t.states.borrow_mut().push(false);
        Self {
            pid: p,
            cmd: c,
            parent: t,
        }
    }
    pub fn skill(self) {
        drop(self)
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid)
    }
}
