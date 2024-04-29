pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Message {
        Message { content, user }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(content) => (true, content),
        None => (false, "ERROR: illegal"),
    }
}
