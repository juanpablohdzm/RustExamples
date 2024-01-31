use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self // or Server
    {
        return Server { address: address };
    }

    pub fn run(self) // this takes ownership of self, self will deallocate after run
    {
        let listener = TcpListener::bind(&self.address).unwrap();
        
        //infinite loop
        loop 
        {
            match listener.accept() {
                Ok((stream, _)) => {

                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}

