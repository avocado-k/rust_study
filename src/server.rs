use std::net::TcpListener;


pub struct Server {
    addr: String,

}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("listening on {}", self.addr);

        let listner = TcpListener::bind(&self.addr);
    }
}