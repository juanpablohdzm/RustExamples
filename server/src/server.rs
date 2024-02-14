use std::{io::Read, net::TcpListener, io::Write};
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpStream;

pub  trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse reques: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self // or Server
    {
        return Server { address: address };
    }

    pub fn run(self, mut handler: impl Handler) // this takes ownership of self, self will deallocate after run
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
                            let response  = match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                },
                            };
                            
                            if let Err(e) = response.send(&mut  stream) {
                                println!("Failed to send response: {}", e)
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

