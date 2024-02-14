use std::{io::Read, net::TcpListener};
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

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
        println!("Listening on address: {}", self.address);
        //infinite loop
        loop 
        {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer)
                    {
                        Ok(_) => { 
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                            
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}

