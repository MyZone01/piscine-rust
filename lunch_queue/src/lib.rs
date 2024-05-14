// ## lunch_queue

// ### Instructions

// You will need to create an _API_, so that a program can organize a queue of people.

// The program requires the following functions. Add them as associated functions to the `Queue` structure:

// - `new`: which will initialize the `Queue`.
// - `add`: which adds a person to the queue.
// - `invert_queue`: which reverses the queue.
// - `rm`: which removes the person who finished ordering their food. The removal should respect the FIFO method (first in first out). It should return the person's details.
// - `search`: which returns the details for a given person's `name`.

// You must also create a type named `Link`. This will be the connection of the structures `Queue` and `Person`. This will be a recursion type, and must point to `None` if there is no `Person` to point to.

// ### Expected Function and Structures

// ```rust
// pub struct Queue {
//     pub node: Link,
// }

// pub type Link =

// pub struct Person {
//     pub discount: i32,
//     pub name: String,
// }

// impl Queue {
//     pub fn new() -> Queue {

//     }
//     pub fn add(&mut self, name: String, discount: i32) {

//     }
//     pub fn invert_queue(&mut self) {

//     }
//     pub fn rm(&mut self) -> Option<(String, i32)> {

//     }
//     pub fn search(&self, name: &str) -> Option<(String, i32)> {

//     }
// }
// ```

// ### Usage

// Here is a program to test your function:

// ```rust
// use lunch_queue::*;

// fn main() {
//     let mut list = Queue::new();
//     list.add(String::from("Marie"), 20);
//     list.add(String::from("Monica"), 15);
//     list.add(String::from("Ana"), 5);
//     list.add(String::from("Alice"), 35);
//     println!("{:?}", list);

//     println!("{:?}", list.search("Marie"));
//     println!("{:?}", list.search("Alice"));
//     println!("{:?}", list.search("someone"));

//     println!("removed {:?}", list.rm());
//     println!("list {:?}", list);
//     list.invert_queue();
//     println!("invert {:?}", list);
// }
// ```

// And its output:

// ```console
// $ cargo run
// Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Marie", discount: 20, next_person: None }) }) }) }) }
// Some(("Marie", 20))
// Some(("Alice", 35))
// None
// removed Some(("Marie", 20))
// list Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: None }) }) }) }
// invert Queue { node: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Alice", discount: 35, next_person: None }) }) }) }
// $
// ```

// ### Notions

// - [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
// - [Box](https://doc.rust-lang.org/book/ch15-01-box.html)
// - [std::option](https://doc.rust-lang.org/std/option/)

#[derive(Debug, Clone)]
pub struct Queue {
    pub node: Link,
    pub last: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            node: None,
            last: None,
        }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let last_node = self.last.take();
        // check if the queue is empty
        if last_node.is_none() {
            self.node = Some(Box::new(Person {
                name: name.clone(),
                discount,
                next_person: None,
            }));
            self.last = Some(Box::new(Person {
                name: name.clone(),
                discount,
                next_person: None,
            }));
        } else if let Some(mut node) = last_node {
            node.next_person = Some(Box::new(Person {
                name: name.clone(),
                discount,
                next_person: None,
            }));
            self.last = Some(node);
        }
        println!("new person: {:?}", name);
        self.print_queue();
    }

    pub fn print_queue(&self) {
        println!("Printing queue");
        let mut current = self.node.as_ref();
        let mut queue_str = String::from(">");
        while let Some(node) = current {
            queue_str.push_str(&node.name);
            if let Some(_next) = node.next_person.as_ref() {
                queue_str.push_str(" -> ");
            }
            current = node.next_person.as_ref();
        }
        println!("{}", queue_str);
    }

    pub fn invert_queue(&mut self) {
        let mut current = self.node.take();
        let mut new_node = None;
        while let Some(mut node) = current {
            let next = node.next_person.take();
            node.next_person = new_node.take();
            new_node = Some(node);
            current = next;
        }
        self.node = new_node;
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        let current = self.node.take();
        match current {
            Some(mut node) => {
                let next = node.next_person.take();
                self.node = next;
                Some((node.name, node.discount))
            }
            None => None,
        }
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = self.node.as_ref();
        while let Some(node) = current {
            if node.name == name {
                return Some((node.name.clone(), node.discount));
            }
            current = node.next_person.as_ref();
        }
        None
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}
