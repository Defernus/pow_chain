use std::net::TcpListener;

pub struct Node {
    listener: TcpListener,
}

impl Node {
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }

    pub fn run(&self) {
        println!("Hello from NODE!");
    }
}
