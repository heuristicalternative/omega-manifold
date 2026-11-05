#![allow(dead_code)]
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Message {
    Heartbeat,
    MessageA,
    MessageB,
}

struct Node {
    messages: HashSet<Message>,
}

impl Node {
    fn new() -> Node {
        Node { messages: HashSet::new(), }
    }

    fn send(&mut self, message: Message) {
        self.messages.insert(message);
    }

    fn listen(&self) -> HashSet<Message> {
        self.messages.clone()
    }
}

fn main() {
    let mut nodes = Node::new();

    // Send heartbeats
    nodes.send(Message::Heartbeat);

    // Listen for messages
    for message in nodes.listen() {
        println!("{:?}", message);
    }

    // Simulate a message reception
    nodes.send(Message::MessageA);
}