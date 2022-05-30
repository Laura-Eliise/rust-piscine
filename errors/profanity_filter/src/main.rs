pub struct Message {
    user: String,
    content: String,
}

impl Message {
  pub fn new(ms: String, u: String) -> Message {
    return Message {
        user: u,
        content: ms,
    }
  }
  pub fn send_ms(&self) -> Option<&str> {
    if self.content == "" || self.content.contains("stupid") {
        return None
    }
    return Some(&self.content)
  }
}

fn main() {
    let m0 = Message::new("hello there".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m0));
  
    let m1 = Message::new("".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m1));
  
    let m2 = Message::new("you are stupid".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m2));
  
    let m3 = Message::new("stupid".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m3));
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    if ms.send_ms() == None {
        return (false, "ERROR: illegal")
    }
    return (true, &ms.content)
}