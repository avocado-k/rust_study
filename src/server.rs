use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::convert::Tryinto;

pub struct Server {
    addr: String,
}


impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; //0 for 1024 bites
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }

                        }
                        Err(e) => println!("failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("failed to establish a connection {}", e),
                //_ => println!("failed to establish a connection"),//default
            }
        }
    }
}