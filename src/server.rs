use std::net::TcpListener;


pub struct Server {
    addr: String,

}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self){
        println!("listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop{
            match listener.accept() {
                OK((stream, _)) =>{
                    let a = 5;
                    println!("ok");
                },
                Err(e) => println!("failed to establish a connection {}",e),
                //_ => println!("failed to establish a connection"),//default
            }
        }
    }
}